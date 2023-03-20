use crate::search::a_star_search;

crate::day!(12, Graph, usize {
    parse_input(input) {
        let nodes = input.lines().enumerate().flat_map(|(line_index, line)| {
            line.chars().enumerate().map(move |(char_index, char)| {
                Node::new(char, char_index as isize, line_index as isize)
            })
        }).collect();
        Graph { nodes, width: input.find("\n").unwrap_or(input.len()) as isize }
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
    width: isize,
}

impl Graph {
    fn neighbours<'a>(&'a self, node: &'a Node) -> Vec<(&'a Node, u32)> {
        [-1, 1, -self.width, self.width]
            .iter()
            .filter_map(|step| {
                let index = node.y * self.width + node.x + step;
                if index >= 0 {
                    self.nodes.get(index as usize)
                } else {
                    None
                }
            })
            .map(|n| (n, 1))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    c: char,
    height: i32,
    x: isize,
    y: isize,
}

impl Node {
    fn new(c: char, x: isize, y: isize) -> Node {
        match c {
            'S' => Node { c, x, y, height: 0 },
            'E' => Node { c, x, y, height: 26 },
            'a'..='z' => Node { c, x, y, height: c as i32 - 'a' as i32 },
            _ => panic!("Unsupported character"),
        }
    }
}
