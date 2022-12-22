/* use std::ops::DerefMut;

use regex::Regex;

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: Option<u16>,
    children: Vec<File>, //todo map gebruiken vr direct access obv naam
}

impl File {
    fn size(&self) -> u32 {
        self.size.unwrap_or(0).into()
    }

    fn dir(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: Option::None,
            children: Vec::new(),
        }
    }

    fn file(name: &str, size: u16) -> Self {
        Self {
            name: name.to_string(),
            size: Option::Some(size),
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: File) {
        self.children.push(child);
    }

    fn get_child_mut(&mut self) -> &mut File {
        self.children.iter_mut().last().expect("durr")
    }

    fn find_child_mut(&mut self, to_open: &str) -> &mut File {
        let found = self.children.iter_mut().find(|child| child.name == to_open);
        match found {
            Some(file) => file,
            None => {
                println!("Could not open {to_open}");
                panic!("ded");
            }
        }
    }
}

fn parse_tree(input: &String) -> File {
    let mut root = File::dir("/");
    let mut path: Vec<&mut File> = Vec::new();
    path.push(&mut root);

    let digits = Regex::new(r"\d+").unwrap();

    input.lines().for_each(|line| match line.trim() {
        "$ cd /" => println!("root command is implicit"),

        "$ ls" => println!("noop, don't care for lists"),

        i if i.starts_with("dir") => {
            let dir = File::dir(&i[4..i.len()]);
            path.last_mut().expect("blerp").add_child(dir);
        }

        i if digits.is_match(i) => {
            let split: Vec<&str> = i.split(" ").collect();
            let file = File::file(
                split[1],
                split[0].parse::<u16>().expect("couldnt parse size"),
            );
            path.last_mut().expect("herp").add_child(file);
        }

        "$ cd .." => {
            path.pop(); //Remove last traversed
        }

        i if i.starts_with("$ cd") => {
            let to_open = &i[4..i.len()];
            let last = path.pop().expect("derp");
            let found = &last.find_child_mut(to_open).deref_mut();
            path.push(last);
            //path.push(found)
            //path.push(found);

            /*
            let mut items = vec![1];
            let item = items.last().cloned();
            items.push(2);
            */
        }

        _ => println!("no matching command for {}", line),
    });

    root
}

#[cfg(test)]
mod tests {
    use super::parse_tree;

    fn input() -> String {
        "$ cd /
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
"
        .to_string()
    }

    #[test]
    fn test_day_a() {
        let input = input();
        let parsed = parse_tree(&input);
        println!("{:#?}", parsed)
    }
}
 */