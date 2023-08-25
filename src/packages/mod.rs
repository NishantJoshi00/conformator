use color_eyre::{eyre::bail, Result};
use std::collections::{HashMap, HashSet};

use crate::types;

fn construct_package_tree<'a: 'b, 'b>(
    packages: &'b types::Packages<'a>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut dependencies = HashSet::new();
    let output: HashMap<_, _> = packages
        .iter()
        .map(|package| {
            let hs_dependency: HashSet<_> = package.dependency.iter().copied().collect();
            dependencies = dependencies.union(&hs_dependency).copied().collect();
            (package.name, hs_dependency)
        })
        .collect();
    dependencies
        .difference(&output.iter().map(|(&key, _)| key).collect::<HashSet<_>>())
        .map(|&x| (x, HashSet::new()))
        .chain(output.into_iter())
        .collect()
}

fn construct_levels<'a>(
    mut package_map: HashMap<&'a str, HashSet<&'a str>>,
) -> Result<Vec<Vec<&'a str>>> {
    if graph_validation(&package_map) {
        bail!("There are cycles present in the dependency graph")
    }
    let mut levels = Vec::new();

    while !package_map.is_empty() {
        let mut removal_list = HashSet::new();

        for (&key, value) in &mut package_map {
            if value.is_empty() {
                removal_list.insert(key);
            }
        }

        removal_list.iter().for_each(|&key| {
            package_map.remove(key);
        });

        package_map.iter_mut().for_each(|(_, value)| {
            *value = value.difference(&removal_list).copied().collect();
        });

        levels.push(removal_list.into_iter().collect());
    }

    Ok(levels)
}

fn visitor<'a: 'b, 'b>(
    current_key: &'a str,
    graph: &'a HashMap<&'a str, HashSet<&'a str>>,
    stack: &'b mut HashSet<&'a str>,
) -> bool {
    if stack.contains(current_key) {
        true
    } else {
        stack.insert(current_key);
        graph
            .get(current_key)
            .map(|x| {
                x.iter().fold(false, |x, &y| {
                    let state = visitor(y, graph, stack);
                    state || x
                })
            })
            .unwrap_or(false)
    }
}

fn graph_validation<'a>(graph: &'a HashMap<&'a str, HashSet<&'a str>>) -> bool {
    graph.iter().fold(false, |acc, (&key, _)| {
        let mut stack = HashSet::new();
        acc || visitor(key, graph, &mut stack)
    })
}

pub fn construct_staged_list(packages: types::Packages) -> Result<Vec<types::Packages>> {
    let map = construct_package_tree(&packages);
    let mut partial_iter: HashSet<_> = map.iter().map(|(&x, _)| x).collect();
    let package_map: HashMap<_, _> = packages
        .into_iter()
        .map(|package| {
            partial_iter.remove(package.name);
            (package.name, package)
        })
        .collect();
    let mut package_map: HashMap<_, _> = package_map
        .into_iter()
        .chain(
            partial_iter
                .iter()
                .map(|&name| (name, types::Package::from_name(name))),
        )
        .collect();

    let stages = construct_levels(map)?;
    Ok(stages
        .into_iter()
        .map(|stage| {
            stage
                .into_iter()
                .filter_map(|key| package_map.remove(key))
                .collect()
        })
        .collect())
}
