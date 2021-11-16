use std::collections::BTreeSet;
use std::env;
use std::path::PathBuf;

use crate::gradle_dependency::GradleDependency;

mod files;
mod gradle_dependency;
mod lockfile;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: PathBuf = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().expect("unable to read current directory")
    };

    let mut files: Vec<PathBuf> = vec![];
    let mut deps: BTreeSet<GradleDependency> = BTreeSet::new();

    match files::find_files(&path, &mut files, lockfile::is_lockfile) {
        Ok(_) => {
            parse_lockfiles(&files, &mut deps);

            for dep in deps {
                println!("{}", dep);
            }
        }

        Err(e) => println!("Error collecting files: {}", e),
    };
}

fn parse_lockfiles(files: &[PathBuf], deps: &mut BTreeSet<GradleDependency>) {
    for file in files {
        if let Ok(lines) = files::read_lines(file) {
            for line in lines.flatten() {
                if let Some(dep) = lockfile::parse_line(line) {
                    deps.insert(dep);
                }
            }
        }
    }
}
