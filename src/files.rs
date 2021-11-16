use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn find_files(
    path: &Path,
    collect: &mut Vec<PathBuf>,
    f: fn(&Path) -> bool,
) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                find_files(&path, collect, f)?;
            } else if f(&path) {
                collect.push(path);
            }
        }
    }

    Ok(())
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
