//! Fails when `bifrost-core` reaches a denied crate through the Cargo graph.
//!
//! The source rules of arch-lint do not follow glob imports, re-exports or
//! macros, and never see a crate that is only a dependency. This check reads
//! the resolved dependency graph from `cargo metadata` instead, so a denied
//! crate fails whether it is a direct or a transitive dependency, and
//! whatever kind of dependency edge brings it in.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::process::Command;

use serde_json::Value;

use crate::config::is_denied;
use crate::error::Error;

/// The package the boundary protects.
pub const PROTECTED: &str = "bifrost-core";

/// Returns one line per denied crate that `bifrost-core` depends on.
///
/// # Errors
///
/// Returns [`Error::Metadata`] when `cargo metadata` fails, its output cannot
/// be parsed, or the workspace has no `bifrost-core` package.
pub fn findings(root: &Path, denied: &[String]) -> Result<Vec<String>, Error> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .arg("--manifest-path")
        .arg(root.join("Cargo.toml"))
        .output()
        .map_err(|source| Error::Metadata(source.to_string()))?;
    if !output.status.success() {
        return Err(Error::Metadata(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }
    let metadata: Value = serde_json::from_slice(&output.stdout)
        .map_err(|source| Error::Metadata(source.to_string()))?;
    findings_in(&metadata, denied)
}

/// Like [`findings`], on metadata that was already read.
///
/// # Errors
///
/// Returns [`Error::Metadata`] when the metadata lacks the packages, the
/// resolved graph or the workspace members, or when `bifrost-core` is not
/// exactly one workspace member. A graph that cannot be read never passes.
pub fn findings_in(metadata: &Value, denied: &[String]) -> Result<Vec<String>, Error> {
    reached(&Graph::parse(metadata)?, denied)
}

/// The resolved dependency graph, keyed by package id.
struct Graph {
    names: HashMap<String, String>,
    edges: HashMap<String, Vec<String>>,
    members: Vec<String>,
}

impl Graph {
    fn parse(metadata: &Value) -> Result<Self, Error> {
        let names = required(metadata, "packages")?
            .iter()
            .map(|package| Ok((string(package, "id")?, string(package, "name")?)))
            .collect::<Result<_, Error>>()?;
        let resolve = metadata.get("resolve").ok_or_else(|| missing("resolve"))?;
        let edges = required(resolve, "nodes")?
            .iter()
            .map(|node| Ok((string(node, "id")?, dependencies(node)?)))
            .collect::<Result<_, Error>>()?;
        let members = required(metadata, "workspace_members")?
            .iter()
            .map(|member| member.as_str().map(str::to_owned))
            .collect::<Option<_>>()
            .ok_or_else(|| missing("workspace_members"))?;
        Ok(Self {
            names,
            edges,
            members,
        })
    }

    fn name<'a>(&'a self, key: &'a str) -> &'a str {
        self.names.get(key).map_or(key, String::as_str)
    }

    /// The id of the one workspace member called `package`.
    fn member(&self, package: &str) -> Result<&str, Error> {
        let mut matching = self
            .members
            .iter()
            .filter(|member| self.name(member) == package);
        match (matching.next(), matching.next()) {
            (Some(member), None) => Ok(member),
            _ => Err(Error::Metadata(format!(
                "`{package}` is not exactly one workspace member"
            ))),
        }
    }
}

fn missing(key: &str) -> Error {
    Error::Metadata(format!("the metadata has no usable `{key}`"))
}

fn required<'a>(value: &'a Value, key: &str) -> Result<&'a [Value], Error> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| missing(key))
}

fn string(value: &Value, key: &str) -> Result<String, Error> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| missing(key))
}

fn dependencies(node: &Value) -> Result<Vec<String>, Error> {
    required(node, "deps")?
        .iter()
        .map(|dependency| string(dependency, "pkg"))
        .collect()
}

/// Walks everything `bifrost-core` depends on and reports the denied crates.
fn reached(graph: &Graph, denied: &[String]) -> Result<Vec<String>, Error> {
    let start = graph.member(PROTECTED)?;
    let parents = shortest_parents(graph, start);
    let mut found: Vec<String> = parents
        .keys()
        .filter(|key| is_denied(denied, graph.name(key)))
        .map(|key| chain(graph, &parents, key))
        .collect();
    found.sort();
    Ok(found)
}

/// Maps every package reachable from `start` to its parent on a shortest
/// path (breadth first), so the chain reported for a crate is the shortest.
fn shortest_parents<'g>(graph: &'g Graph, start: &'g str) -> HashMap<&'g str, &'g str> {
    let mut parents = HashMap::new();
    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(current) = queue.pop_front() {
        let unseen: Vec<&str> = graph
            .edges
            .get(current)
            .into_iter()
            .flatten()
            .map(String::as_str)
            .filter(|next| seen.insert(next))
            .collect();
        parents.extend(unseen.iter().map(|next| (*next, current)));
        queue.extend(unseen);
    }
    parents
}

fn chain(graph: &Graph, parents: &HashMap<&str, &str>, last: &str) -> String {
    let mut names = vec![graph.name(last)];
    let mut current = last;
    while let Some(parent) = parents.get(current) {
        names.push(graph.name(parent));
        current = parent;
    }
    names.reverse();
    format!(
        "Cargo.toml: {PROTECTED} must not depend on `{}` ({})",
        graph.name(last),
        names.join(" -> ")
    )
}
