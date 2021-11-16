use std::option::Option;
use std::path::Path;

use crate::gradle_dependency::GradleDependency;

fn extract_version(version: &str) -> &str {
    if let Some(index) = version.find('=') {
        &version[0..index]
    } else {
        version
    }
}

pub fn parse_line(line: String) -> Option<GradleDependency> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let tokens: Vec<&str> = line.split(':').collect();
    if tokens.len() < 3 {
        return None;
    }

    Some(GradleDependency {
        group: tokens[0].to_owned(),
        name: tokens[1].to_owned(),
        version: extract_version(tokens[2]).to_owned(),
    })
}

pub fn is_lockfile(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        ext == "lockfile"
    } else {
        false
    }
}
