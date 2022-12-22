"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var tools_1 = require("./tools");
//testDay7();
day7();
function day7() {
    var reader = new tools_1.FileReader();
    var values = reader.read('./res/day7')
        .then(function (data) { return reader.asStringList(data, '\n'); })
        .then(function (list) { return parseInput(list); })
        .then(function (tree) {
        setDirSize(tree);
        printTree(tree);
        return doDayA(tree);
    })
        .then(function (result) { return console.log("Day 7A: " + result); })
        .catch(function (err) { return console.error(err); });
    var dayb = reader.read('./res/day7')
        .then(function (data) { return reader.asStringList(data, '\n'); })
        .then(function (list) { return parseInput(list); })
        .then(function (tree) {
        setDirSize(tree);
        printTree(tree);
        return doDayB(tree);
    })
        .then(function (result) { return console.log("Day 7B: " + result); })
        .catch(function (err) { return console.error(err); });
}
function testDay7() {
    var test = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
    var reader = new tools_1.FileReader();
    var list = reader.asStringList(test, '\n');
    var tree = parseInput(list);
    setDirSize(tree);
    printTree(tree);
    var resultA = doDayA(tree);
    console.log("test 7 A: " + resultA);
    var resultB = doDayB(tree);
    console.log("test 7 B: " + resultB);
}
function createDir(name) {
    return {
        name: name,
        size: 0,
        children: [],
    };
}
function createFile(name, size) {
    return {
        name: name,
        size: size,
        children: [],
    };
}
function findChild(file, name) {
    for (var _i = 0, _a = file.children; _i < _a.length; _i++) {
        var child = _a[_i];
        if (child.name == name) {
            return child;
        }
    }
    throw new Error("Cant find child with name " + name);
}
function printTree(file, prefix) {
    if (prefix === void 0) { prefix = ""; }
    console.log(prefix.toString() + file.name + " {size: " + file.size + "}");
    for (var _i = 0, _a = file.children; _i < _a.length; _i++) {
        var child = _a[_i];
        printTree(child, prefix + "\t");
    }
}
function setDirSize(file) {
    var sum = 0;
    for (var _i = 0, _a = file.children; _i < _a.length; _i++) {
        var child = _a[_i];
        if (child.size == 0) {
            var subSize = setDirSize(child);
            sum += subSize;
        }
        else {
            sum += child.size;
        }
    }
    file.size = sum;
    return sum;
}
function doDayA(file) {
    var sum = 0;
    if (file.children.length != 0) {
        if (file.size < 100000) {
            sum += file.size;
        }
    }
    for (var _i = 0, _a = file.children; _i < _a.length; _i++) {
        var child = _a[_i];
        sum += doDayA(child);
    }
    return sum;
}
function doDayB(root) {
    var filesystemMax = 70000000;
    var currentFree = filesystemMax - root.size;
    var toDelete = 30000000 - currentFree;
    return doDayBRec(root, toDelete);
}
function doDayBRec(file, toDelete, minSize) {
    if (minSize === void 0) { minSize = Number.MAX_SAFE_INTEGER; }
    if (file.children.length != 0) {
        if (file.size < minSize && file.size > toDelete) {
            minSize = file.size;
        }
    }
    for (var _i = 0, _a = file.children; _i < _a.length; _i++) {
        var child = _a[_i];
        var childSize = doDayBRec(child, toDelete, minSize);
        if (childSize < minSize && childSize > toDelete) {
            minSize = childSize;
        }
    }
    return minSize;
}
function parseInput(list) {
    var root = createDir("/");
    var currentPath = [];
    currentPath.push(root);
    var regexp = RegExp(/^\d/);
    for (var _i = 0, list_1 = list; _i < list_1.length; _i++) {
        var command = list_1[_i];
        if (command == "$ cd /") {
            //Ignore, we have a root
        }
        else if (command == "$ ls") {
            //Ignore list commands
        }
        else if (command.startsWith("dir")) {
            var latest = currentPath.pop();
            var name = command.slice(4);
            latest.children.push(createDir(name));
            currentPath.push(latest);
        }
        else if (regexp.test(command)) {
            var split = command.split(" ");
            var file = createFile(split[1], parseInt(split[0]));
            var latest = currentPath.pop();
            latest.children.push(file);
            currentPath.push(latest);
        }
        else if (command == "$ cd ..") {
            currentPath.pop();
        }
        else if (command.startsWith("$ cd")) {
            var lastDir = currentPath.pop();
            var newDir = findChild(lastDir, command.slice(5));
            currentPath.push(lastDir);
            currentPath.push(newDir);
        }
        else {
            console.error("Could not do stuff for " + command);
        }
    }
    return root;
}
//# sourceMappingURL=day7.js.map