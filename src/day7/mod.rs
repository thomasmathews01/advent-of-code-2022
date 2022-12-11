use std::ops::DerefMut;

use dendron::{HierarchyEditGrant, Node, Tree};
use dendron::traverse::DftEvent;

pub use input::DAY7_INPUT;

mod input;

// TODO: Make this code less ick.

pub fn get_day7_part1_answer(input: &str) -> u64 {
    let (_, dir_sizes) = get_directory_sizes(input);

    dir_sizes.filter(|x| *x <= 100000).sum()
}

pub fn get_day7_part2_answer(input: &str) -> u64 {
    let (total_size, dir_sizes) = get_directory_sizes(input);
    let required_space = 30000000 - (70000000 - total_size);

    dir_sizes
        .filter(|x| *x >= required_space)
        .min().unwrap()
}

fn get_directory_sizes(input: &str) -> (u64, impl Iterator<Item=u64> + '_) {
    let mut tree = parse_tree(input);
    calculate_all_node_sizes(&mut tree);

    let root_size = tree.root().borrow_data().1.unwrap_or(0);

    (root_size, tree.root()
        .depth_first_traverse()
        .filter_map(|ev| {
            match ev {
                DftEvent::Open(_) => None,
                DftEvent::Close(n) => match n.has_children() {
                    true => n.borrow_data().1.clone(),
                    false => None // If it has no children, it is a file, not a directory and should be excluded
                }
            }
        }))
}

fn parse_tree(input: &str) -> Tree<(&str, Option<u64>)> {
    let root = Node::new_tree(("/", None));
    let grant = root.tree().grant_hierarchy_edit().unwrap();

    let mut current_node = root.clone();

    for (command, mut data) in parse_commands(input).skip(1) { // We skip the jump to root, since we are assuming we start at root.
        match command {
            "cd" => move_to_directory(data.next().unwrap(), &mut current_node),
            "ls" => add_new_children_to_node(data, &current_node, &grant),
            _ => panic!("Unknown command: {:?}", command)
        }
    }

    root.tree()
}

fn move_to_directory(directory_name: &str, node: &mut Node<(&str, Option<u64>)>) {
    match directory_name {
        ".." => *node = node.parent().unwrap(),
        dir => *node = node.children().find(|child| &child.borrow_data().0 == &dir).unwrap()
    };
}

fn parse_commands(input: &str) -> impl Iterator<Item=(&str, impl Iterator<Item=&str>)> + '_ {
    input[2..].split("\n$ ")
        .map(|cmd| (&cmd[..2], cmd[3..].lines()))
}

fn add_new_children_to_node<'a, 'b, 'c>(ls_data: impl Iterator<Item=&'a str>, node: &'b Node<(&'a str, Option<u64>)>, grant: &'c HierarchyEditGrant<(&'a str, Option<u64>)>) {
    for item_data in parse_ls_result(ls_data) {
        node.create_as_last_child(grant, item_data);
    }
}

fn parse_ls_result<'a>(ls_data: impl Iterator<Item=&'a str>) -> impl Iterator<Item=(&'a str, Option<u64>)> {
    ls_data.map(|item_in_directory| {
        // This could be either [size, file_name] or ["dir", dir_name]
        let [a, b] = item_in_directory.split(" ").array_chunks::<2>().next().unwrap();

        match a {
            "dir" => (b, None),
            x => (b, x.parse::<u64>().ok())
        }
    })
}

fn calculate_all_node_sizes(tree: &mut Tree<(&str, Option<u64>)>) {
    let root = tree.root();
    set_value_of_node(root);
}

fn set_value_of_node(node: Node<(&str, Option<u64>)>) {
    let mut acc = 0;
    for child in node.children() {
        if child.borrow_data().1.is_none() {
            set_value_of_node(child.clone());
        }

        let data = child.borrow_data();
        acc += data.1.unwrap_or(0);
    }

    let mut data = node.borrow_data_mut();
    let inner_data = data.deref_mut();
    inner_data.1 = Some(acc);
}

#[cfg(test)]
mod tests {
    use crate::day7::get_day7_part1_answer;

    #[test]
    fn test_example() {
        assert_eq!(95437, get_day7_part1_answer("$ cd /
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
7214296 k"));
    }
}