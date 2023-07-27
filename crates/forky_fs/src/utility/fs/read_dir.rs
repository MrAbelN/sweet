use anyhow::Result;
use glob::*;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// read directory and return all sub directories
pub fn read_dir_recursive(path: PathBuf) -> Vec<PathBuf> {
	_read_dir_recursive(Vec::new(), path)
}
fn _read_dir_recursive(mut acc: Vec<PathBuf>, path: PathBuf) -> Vec<PathBuf> {
	if !path.is_dir() {
		return acc;
	}
	let children = fs::read_dir(&path).unwrap();
	acc.push(path);
	children
		.map(|c| c.unwrap().path())
		.fold(acc, _read_dir_recursive)
}

pub fn is_dir_or_extension(path: &PathBuf, ext: &str) -> bool {
	match path.extension() {
		Some(value) => value.to_str().unwrap() == ext,
		None => path.is_dir(),
	}
}

pub fn parents(path: &PathBuf) -> Vec<PathBuf> {
	let mut acc = Vec::new();
	let mut current = path.clone();
	if path.is_dir() {
		acc.push(path.clone());
	}
	while let Some(parent) = current.parent() {
		acc.push(parent.to_path_buf());
		current = parent.to_path_buf();
	}
	acc
}

pub fn is_dir_or_pattern(path: &PathBuf, pattern: &str) -> bool {
	path.is_dir()
		|| Pattern::new(pattern)
			.unwrap()
			.matches(path.to_str().unwrap())
}

/// get all directories matching a glob pattern, removing duplicates
pub fn directories_matching(pattern: &str) -> Vec<PathBuf> {
	glob(pattern)
		.unwrap()
		.filter_map(|val| val.ok())
		.map(|val| {
			if val.is_dir() {
				val
			} else {
				val.parent().unwrap().to_path_buf()
			}
		})
		.collect::<HashSet<PathBuf>>()
		// .fold(HashSet::new(), |mut acc, val| {
		// 	acc.insert(val);
		// 	acc
		// })
		.iter()
		.map(|path| path.clone())
		.collect::<Vec<_>>()
}

// pub fn dir_contains(path: PathBuf, pattern: &str) -> bool {
// 	let pattern = Pattern::new(pattern).unwrap();
// 	glob::glob_with(
// 		&pattern.to_string(),
// 		glob::MatchOptions {
// 			case_sensitive: false,
// 			require_literal_separator: false,
// 			require_literal_leading_dot: false,
// 		},
// 	)
// 	read_dir_recursive(path)
// 		.iter()
// 		.any(|p| pattern. p.to_str().unwrap().contains(pattern))
// }
