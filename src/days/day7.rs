use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Dir {
    name: String,
    dirs: Vec<Dir>, 
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String, 
    size: u16,
}

impl Size for File {
    fn size(&self) -> u32 {
        self.size.into()
    }

    fn is_dir(&self) -> bool {
        false
    }
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(), //Moet hier Stijn -> debug purposes
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add_dir(&mut self, name: &str) {
        self.dirs.push(Dir::new(&name));
    }

    fn add_file(&mut self, name: &str, size: u16) {
        self.files.push(File{
            name: name.to_string(), 
            size: size
        });
    }
}

impl Size for Dir {
    fn size(&self) -> u32 {
        let dir_size: u32 = self.dirs.iter().map(|dir| dir.size()).sum();
        let file_size: u32 = self.files.iter().map(|file| file.size()).sum();
        dir_size + file_size
    }

    fn is_dir(&self) -> bool {
        true
    }
}

pub trait Size : std::fmt::Debug{
    fn size(&self) -> u32;
    fn is_dir(&self) -> bool;
}

fn parse_tree(input: &String) -> Box<dyn Size> {

    let mut root = Dir::new("/");
    let mut current_node: &mut Dir = &mut root;

    let digits = Regex::new(r"\d+").unwrap();

    input.lines().for_each(|line| {
        match line.trim() {
            "$ cd /" => println!("root command is implicit"),

            "$ ls" => println!("noop, don't care for lists"),

            i if i.starts_with("dir") => {
                current_node.add_dir(&i[4..i.len()]);
            },

            i if digits.is_match(i) => {
                let split:Vec<&str> = line.split(" ").collect();
                current_node.add_file(split[1], split[0].parse::<u16>().expect("couldnt parse size"));
            },

            "$ cd .." => {
                println!("Todo but this is suffering in rust");
            },

            i if i.starts_with("$ cd") => {
                let to_open = &i[4..i.len()];
                let mut opt = &current_node.dirs.iter().find(|dir| dir.name == to_open);
                let mut new_node = opt.as_mut();
                match new_node {
                    Some(i) => {
                        println!("Opening dir {}", i.name);
                        current_node = i;
                    },
                    None => {
                        println!("Cannot open {}", to_open);
                    },
                }
            },

            _ => println!("no matching command for {}", line) 
        }
    });

    Box::new(root)
}

#[cfg(test)]
mod tests {
    use super::parse_tree;


    fn input() -> String {"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
".to_string()
    }

    #[test]
    fn test_day_a() {
        let input = input();
        let parsed = parse_tree(&input);
        println!("{:#?}", parsed)   
    }
}