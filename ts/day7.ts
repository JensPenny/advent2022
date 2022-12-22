import { FileReader } from "./tools";

//testDay7();
day7();

function day7() {
    let reader = new FileReader();

    const values = reader.read('./res/day7')
        .then(data => reader.asStringList(data, '\n'))
        .then(list => parseInput(list))
        .then(tree => {
            setDirSize(tree);
            printTree(tree);
            return doDayA(tree)
        })
        .then(result => console.log("Day 7A: " + result))
        .catch(err => console.error(err))

    const dayb = reader.read('./res/day7')
        .then(data => reader.asStringList(data, '\n'))
        .then(list => parseInput(list))
        .then(tree => {
            setDirSize(tree);
            printTree(tree);
            return doDayB(tree)
        })
        .then(result => console.log("Day 7B: " + result))
        .catch(err => console.error(err))

}

function testDay7() {
    let test = `$ cd /
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
7214296 k`
    let reader = new FileReader();
    let list = reader.asStringList(test, '\n');
    let tree = parseInput(list);
    setDirSize(tree);
    printTree(tree);
    let resultA = doDayA(tree);
    console.log("test 7 A: " + resultA);
    let resultB = doDayB(tree);
    console.log("test 7 B: " + resultB);
}

interface File {
    readonly name: String;
    size: number;
    children: File[];
}

function createDir(name: String): File {
    return {
        name: name,
        size: 0,
        children: [],
    }
}

function createFile(name: String, size: number): File {
    return {
        name: name,
        size: size,
        children: [],
    }
}

function findChild(file: File, name: String): File {
    for (let child of file.children) {
        if (child.name == name) {
            return child;
        }
    }
    throw new Error("Cant find child with name " + name);
}

function printTree(file: File, prefix: String = "") {
    console.log(prefix.toString() + file.name + " {size: " + file.size + "}");
    for (let child of file.children) {
        printTree(child, prefix + "\t");
    }
}

function setDirSize(file: File): number {
    let sum = 0;

    for (let child of file.children) {
        if (child.size == 0) {
            let subSize = setDirSize(child);
            sum += subSize;
        } else {
            sum += child.size;
        }
    }

    file.size = sum;
    return sum;
}

function doDayA(file: File): number {
    let sum = 0;

    if (file.children.length != 0) {
        if (file.size < 100000) {
            sum += file.size;
        }
    }
    for (let child of file.children) {
        sum += doDayA(child);
    }

    return sum;
}

function doDayB(root: File) : number {
    const filesystemMax = 70_000_000;
    const currentFree = filesystemMax - root.size;
    const toDelete = 30_000_000 - currentFree;

    return doDayBRec(root, toDelete);
}

function doDayBRec(file: File, toDelete: number, minSize: number = Number.MAX_SAFE_INTEGER): number {

    if (file.children.length != 0) {
        if (file.size < minSize && file.size > toDelete) {
            minSize = file.size;
        }
    }

    for (let child of file.children) {
        let childSize = doDayBRec(child, toDelete, minSize);
        if (childSize < minSize && childSize > toDelete) {
            minSize = childSize;
        }
    }

    return minSize;
}

function parseInput(list: string[]): File {
    const root = createDir("/");
    const currentPath: File[] = [];
    currentPath.push(root);

    const regexp = RegExp(/^\d/);

    for (let command of list) {
        if (command == "$ cd /") {
            //Ignore, we have a root
        } else if (command == "$ ls") {
            //Ignore list commands
        } else if (command.startsWith("dir")) {
            let latest = currentPath.pop()!;
            let name = command.slice(4);
            latest.children.push(createDir(name));
            currentPath.push(latest);
        } else if (regexp.test(command)) {
            let split = command.split(" ");
            let file = createFile(split[1], parseInt(split[0]));
            let latest = currentPath.pop()!;
            latest.children.push(file);
            currentPath.push(latest);
        } else if (command == "$ cd ..") {
            currentPath.pop();
        } else if (command.startsWith("$ cd")) {
            let lastDir = currentPath.pop()!;
            let newDir = findChild(lastDir, command.slice(5));
            currentPath.push(lastDir);
            currentPath.push(newDir)
        } else {
            console.error("Could not do stuff for " + command);
        }
    }

    return root;
}