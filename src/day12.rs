use crate::search::a_star_search;

crate::day!(12, Graph, usize {
    parse_input(input) {
        let nodes: Vec<_> = input.lines().enumerate().flat_map(|(line_index, line)| {
            line.chars().enumerate().map(move |(char_index, char)| {
                Node::new(char, char_index, line_index)
            })
        }).collect();
        let width = nodes.iter().map(|n| n.x).max().unwrap() + 1;
        let height = nodes.iter().map(|n| n.y).max().unwrap() + 1;
        Graph { nodes, width, height }
    }

    calculate_part1(input) {
        struct GraphPart1<'a> {
            graph: &'a Graph
        }

        impl<'a> crate::search::SearchGraph<'a> for GraphPart1<'a> {
            type Node = Node;
            type Score = u32;

            fn neighbours(&self, node: &'a Node) -> Vec<(&'a Node, u32)> {
                self.graph.neighbours(node).iter().cloned().filter(|(next, _)| {
                    next.height - node.height <= 1
                }).collect()
            }

            fn start_node(&self) -> &'a Node {
                self.graph.nodes.iter().find(|n| n.c == 'S').expect("Find the starting node")
            }

            fn is_goal(&self, node: &Node) -> bool { node.c == 'E' }
        }

        a_star_search(&GraphPart1{ graph: input}).expect("Expected a path from S to E").len() - 1
    }

    calculate_part2(input) {
        struct GraphPart2<'a> {
            graph: &'a Graph
        }

        impl<'a> crate::search::SearchGraph<'a> for GraphPart2<'a> {
            type Node = Node;
            type Score = u32;

            fn neighbours(&self, node: &'a Node) -> Vec<(&'a Node, u32)> {
                self.graph.neighbours(node).iter().cloned().filter(|(next, _)| {
                    next.height - node.height >= -1
                }).collect()
            }

            fn start_node(&self) -> &'a Node {
                self.graph.nodes.iter().find(|n| n.c == 'E').expect("Find the starting node")
            }

            fn is_goal(&self, node: &'a Node) -> bool { node.height == 0 }
        }

        a_star_search(&GraphPart2{ graph: input}).expect("Expected a path from E to 0").len() - 1
    }

    test example_input(include_str!("example_input/day12.txt") => 31, 29)
});

struct Graph {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
}

impl Graph {
    fn neighbours<'a>(&'a self, node: &'a Node) -> Vec<(&'a Node, u32)> {
        let mut neighbours = Vec::with_capacity(4);
        if node.x > 0 {
            neighbours.push((self.nodes.get (node.y * self.width + (node.x - 1)).unwrap(), 1));
        }
        if node.x < (self.width - 1) {
            neighbours.push((self.nodes.get (node.y * self.width + (node.x + 1)).unwrap(), 1));
        }
        if node.y > 0 {
            neighbours.push((self.nodes.get ((node.y - 1) * self.width + node.x).unwrap(), 1));
        }
        if node.y < (self.height - 1) {
            neighbours.push((self.nodes.get ((node.y + 1) * self.width + node.x).unwrap(), 1));
        }
        neighbours
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    c: char,
    height: i32,
    x: usize,
    y: usize,
}

impl Node {
    fn new(c: char, x: usize, y: usize) -> Node {
        match c {
            'S' => Node { c, x, y, height: 0 },
            'E' => Node { c, x, y, height: 26 },
            'a'..='z' => Node { c, x, y, height: c as i32 - 'a' as i32 },
            _ => panic!("Unsupported character"),
        }
    }
}
