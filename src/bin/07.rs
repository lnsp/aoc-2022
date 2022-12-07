use std::{io, borrow::Borrow};

use itertools::Itertools;

struct Node {
    id: usize,
    name: String,
    children: Vec<usize>,
    parent: usize,
    size: u64,
}

trait PushReturn<T> {
    fn push_return(&mut self, t: T) -> &mut T;
}

impl<T> PushReturn<T> for Vec<T> {
    fn push_return(&mut self, t: T) -> &mut T {
        self.push(t);
        self.last_mut().unwrap()
    }
}

struct Indexed<I, J>
where
    I: Iterator,
    J: Iterator,
    J::Item: Borrow<usize>,
{
    base: I,
    count: usize,
    index: J,
}

trait IndexedExt: Iterator {
    fn indexed_by<J>(self, index: J) -> Indexed<Self, J>
    where
        Self: Sized,
        J: Iterator,
        J::Item: Borrow<usize>,
    {
        Indexed {
            base: self,
            count: 0,
            index: index,
        }
    }
}

impl<I: Iterator> IndexedExt for I {}

impl<I, J> Iterator for Indexed<I, J>
where
    I: Iterator,
    J: Iterator,
    J::Item: Borrow<usize>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let target = self.index.next();
        let chosen = match target {
            None => None,
            Some(target) => {
                let mut candidate = self.base.next();
                self.count += 1;
                while self.count <= *target.borrow() {
                    candidate = self.base.next();
                    self.count += 1;
                }
                candidate
            }
        };
        return chosen;
    }
}

fn main() -> io::Result<()> {
    let mut files = Vec::<Node>::new();
    files.push_return(Node {
        id: 0,
        name: String::from("/"),
        children: Vec::new(),
        parent: 0,
        size: 0,
    });
    let mut current: usize = 0;


    // 1st stage: build file tree

    let mut buf = String::new();
    while io::stdin().read_line(&mut buf)? > 0 {
        let line = buf.trim();
        if line.starts_with("$ cd ") {
            current = match line.trim_start_matches("$ cd ") {
                "/" => 0,
                ".." => files[current].parent,
                dir => {
                    files
                        .iter()
                        .indexed_by(files[current].children.iter())
                        .find(|d| d.name == dir)
                        .unwrap()
                        .id
                }
            };
        } else if line.starts_with("$ ls") {
            // do nothing, just wait for next line
        } else if line.starts_with("dir ") {
            let id = files.len();
            files.push(Node {
                id: id,
                name: String::from(line.trim_start_matches("dir ")),
                children: Vec::new(),
                parent: current,
                size: 0,
            });
            files[current].children.push(id);
        } else {
            let id = files.len();
            let (size_str, name_str) = line.split_whitespace().collect_tuple().unwrap();
            files.push(Node {
                id: id,
                name: String::from(name_str),
                children: Vec::new(),
                parent: current,
                size: size_str.parse().unwrap(),
            });
            files[current].children.push(id);
        }
        buf.clear();
    };

    // 2nd stage: compute dir sizes
    for fid in (0..files.len()).rev() {
        files[fid].size += files.iter().indexed_by(files[fid].children.iter()).map(|x| x.size).sum::<u64>();
    };

    // 3rd stage (A): sum up directories of size < 10000
    // println!("{}", files.iter().filter(|d| !d.children.is_empty()).filter(|d| d.size <= 100000).map(|d| d.size).sum::<u64>());

    // 3rd stgae (B): find smallest directory such that
    let mut candidate_dirs: Vec<&Node> = files.iter().filter(|d| !d.children.is_empty()).filter(|d| files[0].size - d.size < 40000000).collect();
    candidate_dirs.sort_by(|x, y| x.size.cmp(&y.size));
    println!("{}", candidate_dirs[0].size);

    Ok(())
}
