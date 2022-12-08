const DISK_SPACE_CAPACITY: u32 = 70_000_000;
const DISK_SPACE_NEEDED: u32 = 30_000_000;

#[derive(Debug, PartialEq)]
enum Type {
    Directory,
    File
}

// Inspired by https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
#[derive(Debug)]
struct Node<T>
where
    T: PartialEq + std::fmt::Debug,
{
    idx: usize,
    name: T,
    node_size: u32,
    file_type: Option<Type>,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn new(idx: usize, name: T, node_size: u32, file_type: Option<Type>) -> Self {
        Self {
            idx,
            name,
            node_size,
            file_type,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq + std::fmt::Debug,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn node(&mut self, name: T, node_size: u32, file_type: Option<Type>) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name, node_size, file_type));
        idx
    }

    fn get_directory_size(&self, idx: usize) -> u32 {
        let mut ret = 0;
        for p in &self.arena[idx].children {
            ret += self.get_directory_size(*p);
        }
        ret + self.arena[idx].node_size
    }

    fn get_directories(&self) -> Vec<&Node<T>> {
        self.arena
            .iter()
            .filter(|node| {
                node.file_type == Some(Type::Directory)
            })
            .collect::<Vec<&Node<T>>>()
    }
}

pub fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        .skip(2)
        .collect::<Vec<&str>>()
}

pub fn get_tree(input: &str) -> ArenaTree<String> {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let root = tree.node("/".into(), 0, Some(Type::Directory));
    let mut pwd = root;
    let mut current_dir = root;
    for line in parse(input) {
        let mut split_line: Vec<&str> = line.split_whitespace().collect();
        split_line = split_line[(split_line.len() - 2)..].to_vec();
        match split_line[0] {
            "dir" => {
                let dir = tree.node(
                    format!("{}{}/", tree.arena[pwd].name, split_line[1]),
                    0,
                    Some(Type::Directory)
                );
                tree.arena[pwd].children.push(dir);
                tree.arena[dir].parent = Some(pwd);
            },
            "cd" => {
                if split_line[1] == ".." {
                    current_dir = tree.arena[current_dir].parent.unwrap();
                } else {
                    let dir = tree.node(
                        format!("{}{}/", tree.arena[pwd].name, split_line[1]),
                        0,
                        Some(Type::Directory)
                    );
                    current_dir = dir;
                }
                pwd = current_dir;
            },
            "$" => (),
            &_ => {
                let file_size = split_line[0].parse::<u32>().unwrap();
                let file = tree.node(
                    format!("{}{}", tree.arena[current_dir].name, split_line[1]),
                    file_size,
                    Some(Type::File)
                );
                tree.arena[current_dir].children.push(file);
                tree.arena[file].parent = Some(current_dir);
            }
        }
    }
    tree
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = get_tree(input);
    let directories = tree.get_directories();
    let directory_sizes = directories
        .iter()
        .map(|node| {
            tree.get_directory_size(node.idx)
        })
        .filter(|directory_size| {
            *directory_size <= 100_000
        })
        .collect::<Vec<u32>>();

    Some(directory_sizes.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree = get_tree(input);
    let unused_disk_space = DISK_SPACE_CAPACITY - tree.get_directory_size(0);
    let needed_disk_space = DISK_SPACE_NEEDED - unused_disk_space;
    let directories = tree.get_directories();
    let directory_sizes = directories
        .iter()
        .map(|node| {
            tree.get_directory_size(node.idx)
        })
        .filter(|directory_size| {
            *directory_size >= needed_disk_space
        })
        .collect::<Vec<u32>>();

    Some(*directory_sizes.iter().min().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24_933_642));
    }
}
