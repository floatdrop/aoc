use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

static INPUT: &str = std::include_str!("input.txt");

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;

pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub type EdgeIndex = usize;

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new(),
                edges: Vec::new(), }
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_outgoing_edge: None });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
        edge_index
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }

    pub fn outgoing_edges(&self, source: NodeIndex) -> OutgoingEdges {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        OutgoingEdges { graph: self, current_edge_index: first_outgoing_edge }
    }

    pub fn bfs(&self, start: NodeIndex) -> BFS {
        BFS { graph: self, frontier: VecDeque::from(vec![start]), visited: HashSet::new() }
    }

    pub fn is_reachable(&self, source: NodeIndex, target: NodeIndex) -> bool {
        return self.bfs(source).any(|n| n == target)
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

pub struct OutgoingEdges<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for OutgoingEdges<'graph> {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<EdgeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge_num)
            }
        }
    }
}

pub struct BFS<'graph> {
    graph: &'graph Graph,
    frontier: VecDeque<NodeIndex>,
    visited: HashSet<NodeIndex>
}

impl<'graph> Iterator for BFS<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        if self.frontier.is_empty() {
            return None;
        }

        let p = self.frontier.pop_front().unwrap();

        for n in self.graph.successors(p) {
            if !self.visited.contains(&n) {
                self.visited.insert(n);
                self.frontier.push_back(n);
            }
        }

        Some(p)
    }
}

pub fn part1() -> usize {
    let mut graph = Graph::new();
    let mut color_indexes: HashMap<String, NodeIndex> = HashMap::new();

    for line in INPUT.split("\n") {
        if line.ends_with("no other bags.") {
            continue;
        }
        
        let mut words = line.split_whitespace().into_iter();
        
        let parent_color = format!("{} {}", words.next().unwrap(), words.next().unwrap());
        words.next(); // bags
        words.next(); // contains
        
        loop {
            let _amount: usize = words.next().unwrap().parse().unwrap();
            let amount_color = format!("{} {}", words.next().unwrap(), words.next().unwrap());

            if !color_indexes.contains_key(&parent_color) {
                color_indexes.insert(parent_color.clone(), graph.add_node());
            }

            if !color_indexes.contains_key(&amount_color) {
                color_indexes.insert(amount_color.clone(), graph.add_node());
            }

            graph.add_edge(color_indexes[parent_color.as_str()], color_indexes[amount_color.as_str()]);

            if words.next().unwrap().ends_with(".") {
                break;
            }
        }
    }

    let target = color_indexes["shiny gold"];
    color_indexes.iter().filter(|(k, &source)| (k != &"shiny gold") && (graph.is_reachable(source, target))).count()
}

pub fn part2() -> usize {
    let mut graph = Graph::new();
    let mut color_indexes: HashMap<String, NodeIndex> = HashMap::new();
    let mut edge_weights: HashMap<EdgeIndex, usize> = HashMap::new();

    for line in INPUT.split("\n") {
        if line.ends_with("no other bags.") {
            continue;
        }
        
        let mut words = line.split_whitespace().into_iter();
        
        let parent_color = format!("{} {}", words.next().unwrap(), words.next().unwrap());
        words.next(); // bags
        words.next(); // contains
        
        loop {
            let amount: usize = words.next().unwrap().parse().unwrap();
            let amount_color = format!("{} {}", words.next().unwrap(), words.next().unwrap());

            if !color_indexes.contains_key(&parent_color) {
                color_indexes.insert(parent_color.clone(), graph.add_node());
            }

            if !color_indexes.contains_key(&amount_color) {
                color_indexes.insert(amount_color.clone(), graph.add_node());
            }

            let edge = graph.add_edge(color_indexes[parent_color.as_str()], color_indexes[amount_color.as_str()]);
            edge_weights.insert(edge, amount);

            if words.next().unwrap().ends_with(".") {
                break;
            }
        }
    }

    let source = color_indexes["shiny gold"];
    let mut sum: usize = 0;
    let mut frontier: VecDeque<Vec<EdgeIndex>> = graph.outgoing_edges(source).map(|e| vec![e]).collect();

    while !frontier.is_empty() {
        let path = frontier.pop_back().unwrap();
        let e = path.last().unwrap();
        let p = graph.edges[*e].target;
        
        sum += path.iter().map(|i| edge_weights[&i]).product::<usize>();
        
        for n in graph.outgoing_edges(p) {
            let mut new_path = path.clone();
            new_path.push(n);
            frontier.push_back(new_path);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1_answer() {
        assert_eq!(part1(), 252);
    }
    
    #[test]
    fn check_part2_answer() {
        assert_eq!(part2(), 35487);
    }
}
