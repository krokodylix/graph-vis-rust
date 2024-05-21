use wasm_bindgen::prelude::*;
use js_sys::Math;

// Define Point structure
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Define Node structure
#[derive(Debug)]
struct Node {
    position: Point,
    disp: Point,
}

// Define Edge structure
#[derive(Debug)]
struct Edge {
    source: usize,
    target: usize,
}

// Define Graph structure
#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    // Initialize a new Graph
    fn new(num_nodes: usize, edges: Vec<Edge>) -> Graph {
        let nodes = (0..num_nodes)
            .map(|_| Node {
                position: Point {
                    x: Math::random() * 100.0,
                    y: Math::random() * 100.0,
                },
                disp: Point { x: 0.0, y: 0.0 },
            })
            .collect();
        Graph { nodes, edges }
    }

    // Create Graph from a string
    fn from_string(graph_str: &str) -> Graph {
        let edges: Vec<Edge> = graph_str
            .split(',')
            .map(|s| {
                let nodes: Vec<usize> = s.split('-')
                    .map(|n| n.parse().unwrap())
                    .collect();
                Edge { source: nodes[0], target: nodes[1] }
            })
            .collect();

        let num_nodes = edges.iter()
            .flat_map(|e| vec![e.source, e.target])
            .max()
            .map_or(0, |max_node| max_node + 1);

        Graph::new(num_nodes, edges)
    }

    // Implement Force-Atlas2 algorithm
    fn force_atlas2(&mut self, iterations: usize, gravity: f64, scaling_ratio: f64) {
        for _ in 0..iterations {
            // Reset displacement
            for node in &mut self.nodes {
                node.disp = Point { x: 0.0, y: 0.0 };
            }

            // Calculate repulsive forces
            for i in 0..self.nodes.len() {
                for j in 0..self.nodes.len() {
                    if i != j {
                        let delta = Point {
                            x: self.nodes[i].position.x - self.nodes[j].position.x,
                            y: self.nodes[i].position.y - self.nodes[j].position.y,
                        };
                        let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
                        if distance > 0.0 {
                            let repulsive_force = scaling_ratio / distance;
                            self.nodes[i].disp.x += delta.x / distance * repulsive_force;
                            self.nodes[i].disp.y += delta.y / distance * repulsive_force;
                        }
                    }
                }
            }

            // Calculate attractive forces
            for edge in &self.edges {
                let delta = Point {
                    x: self.nodes[edge.source].position.x - self.nodes[edge.target].position.x,
                    y: self.nodes[edge.source].position.y - self.nodes[edge.target].position.y,
                };
                let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
                if distance > 0.0 {
                    let attractive_force = distance * distance / scaling_ratio;
                    self.nodes[edge.source].disp.x -= delta.x / distance * attractive_force;
                    self.nodes[edge.source].disp.y -= delta.y / distance * attractive_force;
                    self.nodes[edge.target].disp.x += delta.x / distance * attractive_force;
                    self.nodes[edge.target].disp.y += delta.y / distance * attractive_force;
                }
            }

            // Apply gravity
            for node in &mut self.nodes {
                let distance_to_center = (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
                node.disp.x -= node.position.x * gravity / distance_to_center;
                node.disp.y -= node.position.y * gravity / distance_to_center;
            }

            // Update positions
            for node in &mut self.nodes {
                let disp_length = (node.disp.x * node.disp.x + node.disp.y * node.disp.y).sqrt();
                if disp_length > 0.0 {
                    node.position.x += node.disp.x / disp_length * disp_length.min(1.0);
                    node.position.y += node.disp.y / disp_length * disp_length.min(1.0);
                }

                // Prevent nodes from moving too far away
                node.position.x = node.position.x.max(0.0).min(100.0);
                node.position.y = node.position.y.max(0.0).min(100.0);
            }
        }
    }

    // Convert Graph to a string
    fn to_string(&self) -> String {
        self.edges.iter()
            .map(|e| format!("{}-{}", e.source, e.target))
            .collect::<Vec<_>>()
            .join(",")
    }
}

// WASM Bindgen to expose the function to JavaScript
#[wasm_bindgen]
pub fn process_graph(graph_str: &str, iterations: usize, gravity: f64, scaling_ratio: f64) -> String {
    let mut graph = Graph::from_string(graph_str);
    graph.force_atlas2(iterations, gravity, scaling_ratio);
    graph.to_string()
}
