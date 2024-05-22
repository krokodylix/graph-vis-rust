use wasm_bindgen::prelude::*;
use js_sys::Math;
use std::f64::consts::PI;

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

// Initialize a new Graph
fn new_graph(num_nodes: usize, edges: Vec<Edge>) -> Graph {
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

    new_graph(num_nodes, edges)
}

// Implement Force-Atlas2 algorithm
fn force_atlas2(graph: &mut Graph, iterations: usize, gravity: f64, scaling_ratio: f64) -> &Graph {
    for _ in 0..iterations {
        // Reset displacement
        for node in &mut graph.nodes {
            node.disp = Point { x: 0.0, y: 0.0 };
        }

        // Calculate repulsive forces
        for i in 0..graph.nodes.len() {
            for j in 0..graph.nodes.len() {
                if i != j {
                    let delta = Point {
                        x: graph.nodes[i].position.x - graph.nodes[j].position.x,
                        y: graph.nodes[i].position.y - graph.nodes[j].position.y,
                    };
                    let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
                    if distance > 0.0 {
                        let repulsive_force = scaling_ratio / distance;
                        graph.nodes[i].disp.x += delta.x / distance * repulsive_force;
                        graph.nodes[i].disp.y += delta.y / distance * repulsive_force;
                    }
                }
            }
        }

        // Calculate attractive forces
        for edge in &graph.edges {
            let delta = Point {
                x: graph.nodes[edge.source].position.x - graph.nodes[edge.target].position.x,
                y: graph.nodes[edge.source].position.y - graph.nodes[edge.target].position.y,
            };
            let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
            if distance > 0.0 {
                let attractive_force = distance * distance / scaling_ratio;
                graph.nodes[edge.source].disp.x -= delta.x / distance * attractive_force;
                graph.nodes[edge.source].disp.y -= delta.y / distance * attractive_force;
                graph.nodes[edge.target].disp.x += delta.x / distance * attractive_force;
                graph.nodes[edge.target].disp.y += delta.y / distance * attractive_force;
            }
        }

        // Apply gravity
        for node in &mut graph.nodes {
            let distance_to_center = (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
            node.disp.x -= node.position.x * gravity / distance_to_center;
            node.disp.y -= node.position.y * gravity / distance_to_center;
        }

        // Update positions
        for node in &mut graph.nodes {
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
    graph
}

// Implement Circular Layout
fn circular_layout(graph: &mut Graph) -> &Graph {
    let num_nodes = graph.nodes.len();
    let radius = 50.0;
    for (i, node) in graph.nodes.iter_mut().enumerate() {
        let angle = (i as f64 / num_nodes as f64) * 2.0 * PI;
        node.position = Point {
            x: 50.0 + radius * angle.cos(),
            y: 50.0 + radius * angle.sin(),
        };
    }
    graph
}

// Implement Random Layout
fn random_layout(graph: &mut Graph) -> &Graph {
    for node in &mut graph.nodes {
        node.position = Point {
            x: Math::random() * 100.0,
            y: Math::random() * 100.0,
        };
    }
    graph
}

// Implement Fruchterman-Reingold Algorithm
fn fruchterman_reingold(graph: &mut Graph, iterations: usize, area: f64, gravity: f64) -> &Graph {
    let k = (area / graph.nodes.len() as f64).sqrt();

    for _ in 0..iterations {
        // Reset displacement
        for node in &mut graph.nodes {
            node.disp = Point { x: 0.0, y: 0.0 };
        }

        // Calculate repulsive forces
        for i in 0..graph.nodes.len() {
            for j in 0..graph.nodes.len() {
                if i != j {
                    let delta = Point {
                        x: graph.nodes[i].position.x - graph.nodes[j].position.x,
                        y: graph.nodes[i].position.y - graph.nodes[j].position.y,
                    };
                    let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
                    if distance > 0.0 {
                        let repulsive_force = k * k / distance;
                        graph.nodes[i].disp.x += delta.x / distance * repulsive_force;
                        graph.nodes[i].disp.y += delta.y / distance * repulsive_force;
                    }
                }
            }
        }

        // Calculate attractive forces
        for edge in &graph.edges {
            let delta = Point {
                x: graph.nodes[edge.source].position.x - graph.nodes[edge.target].position.x,
                y: graph.nodes[edge.source].position.y - graph.nodes[edge.target].position.y,
            };
            let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
            if distance > 0.0 {
                let attractive_force = distance * distance / k;
                graph.nodes[edge.source].disp.x -= delta.x / distance * attractive_force;
                graph.nodes[edge.source].disp.y -= delta.y / distance * attractive_force;
                graph.nodes[edge.target].disp.x += delta.x / distance * attractive_force;
                graph.nodes[edge.target].disp.y += delta.y / distance * attractive_force;
            }
        }

        // Apply gravity
        for node in &mut graph.nodes {
            let distance_to_center = (node.position.x * node.position.x + node.position.y * node.position.y).sqrt();
            node.disp.x -= node.position.x * gravity * distance_to_center / k;
            node.disp.y -= node.position.y * gravity * distance_to_center / k;
        }

        // Update positions
        for node in &mut graph.nodes {
            let disp_length = (node.disp.x * node.disp.x + node.disp.y * node.disp.y).sqrt();
            if disp_length > 0.0 {
                node.position.x += node.disp.x / disp_length * disp_length.min(k);
                node.position.y += node.disp.y / disp_length * disp_length.min(k);
            }

            // Prevent nodes from moving too far away
            node.position.x = node.position.x.max(0.0).min(100.0);
            node.position.y = node.position.y.max(0.0).min(100.0);
        }
    }
    graph
}

// Implement Stress Majorization Algorithm
fn stress_majorization(graph: &mut Graph, iterations: usize) -> &Graph {
    let mut distances = vec![vec![f64::INFINITY; graph.nodes.len()]; graph.nodes.len()];

    // Compute shortest path distances (Floyd-Warshall Algorithm)
    for i in 0..graph.nodes.len() {
        distances[i][i] = 0.0;
    }
    for edge in &graph.edges {
        distances[edge.source][edge.target] = 1.0;
        distances[edge.target][edge.source] = 1.0;
    }
    for k in 0..graph.nodes.len() {
        for i in 0..graph.nodes.len() {
            for j in 0..graph.nodes.len() {
                let new_distance = distances[i][k] + distances[k][j];
                if new_distance < distances[i][j] {
                    distances[i][j] = new_distance;
                }
            }
        }
    }

    // Initialize positions randomly
    for node in &mut graph.nodes {
        node.position = Point {
            x: Math::random() * 100.0,
            y: Math::random() * 100.0,
        };
    }

    // Stress majorization iterations
    for _ in 0..iterations {
        for i in 0..graph.nodes.len() {
            let mut new_position = Point { x: 0.0, y: 0.0 };
            let mut weight_sum = 0.0;

            for j in 0..graph.nodes.len() {
                if i != j {
                    let delta = Point {
                        x: graph.nodes[i].position.x - graph.nodes[j].position.x,
                        y: graph.nodes[i].position.y - graph.nodes[j].position.y,
                    };
                    let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
                    let ideal_distance = distances[i][j];
                    if distance > 0.0 && ideal_distance < f64::INFINITY {
                        let weight = 1.0 / (ideal_distance * ideal_distance);
                        new_position.x += weight * (graph.nodes[j].position.x + delta.x * ideal_distance / distance);
                        new_position.y += weight * (graph.nodes[j].position.y + delta.y * ideal_distance / distance);
                        weight_sum += weight;
                    }
                }
            }

            graph.nodes[i].position.x = new_position.x / weight_sum;
            graph.nodes[i].position.y = new_position.y / weight_sum;
        }
    }
    graph
}

// Implement Multidimensional Scaling (MDS) Algorithm
fn multidimensional_scaling(graph: &mut Graph, iterations: usize) -> &Graph {
    let mut distances = vec![vec![f64::INFINITY; graph.nodes.len()]; graph.nodes.len()];

    // Compute shortest path distances (Floyd-Warshall Algorithm)
    for i in 0..graph.nodes.len() {
        distances[i][i] = 0.0;
    }
    for edge in &graph.edges {
        distances[edge.source][edge.target] = 1.0;
        distances[edge.target][edge.source] = 1.0;
    }
    for k in 0..graph.nodes.len() {
        for i in 0..graph.nodes.len() {
            for j in 0..graph.nodes.len() {
                let new_distance = distances[i][k] + distances[k][j];
                if new_distance < distances[i][j] {
                    distances[i][j] = new_distance;
                }
            }
        }
    }

    // Initialize positions randomly
    for node in &mut graph.nodes {
        node.position = Point {
            x: Math::random() * 100.0,
            y: Math::random() * 100.0,
        };
    }

    // MDS iterations
    for _ in 0..iterations {
        for i in 0..graph.nodes.len() {
            for j in 0..graph.nodes.len() {
                if i != j {
                    let delta = Point {
                        x: graph.nodes[i].position.x - graph.nodes[j].position.x,
                        y: graph.nodes[i].position.y - graph.nodes[j].position.y,
                    };
                    let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
                    let ideal_distance = distances[i][j];
                    if distance > 0.0 && ideal_distance < f64::INFINITY {
                        let weight = 1.0 / (ideal_distance * ideal_distance);
                        graph.nodes[i].position.x += weight * (graph.nodes[j].position.x - graph.nodes[i].position.x) / distance * (distance - ideal_distance);
                        graph.nodes[i].position.y += weight * (graph.nodes[j].position.y - graph.nodes[i].position.y) / distance * (distance - ideal_distance);
                    }
                }
            }
        }
    }
    graph
}


// Convert Graph to a string
fn graph_to_string(graph: &Graph) -> String {
    let mut graph_str = String::new();
    graph_str.push_str("nodes: ");
    for node in &graph.nodes {
        graph_str.push_str(&format!("{},{};", node.position.x, node.position.y));
    }
    graph_str.push_str("edges: ");
    for edge in &graph.edges {
        graph_str.push_str(&format!("{}-{},", edge.source, edge.target));
    }
    graph_str
}

#[wasm_bindgen]
extern "C" {
    fn alert(nodes: &str);
}

// WASM Bindgen to expose the individual functions to JavaScript

#[wasm_bindgen]
pub fn process_random(graph_str: &str) -> String {
    let mut graph = from_string(graph_str);
    random_layout(&mut graph);
    graph_to_string(&graph)
}

#[wasm_bindgen]
pub fn process_force_atlas2(graph_str: &str, iterations: usize, gravity: f64, scaling_ratio: f64) -> String {
    let mut graph = from_string(graph_str);
    force_atlas2(&mut graph, iterations, gravity, scaling_ratio);
    graph_to_string(&graph)
}

#[wasm_bindgen]
pub fn process_circular(graph_str: &str) -> String {
    let mut graph = from_string(graph_str);
    circular_layout(&mut graph);
    graph_to_string(&graph)
}

#[wasm_bindgen]
pub fn process_fruchterman_reingold(graph_str: &str, iterations: usize, gravity: f64) -> String {
    let mut graph = from_string(graph_str);
    fruchterman_reingold(&mut graph, iterations, 10000.0, gravity); // Adjust area parameter as needed
    graph_to_string(&graph)
}

#[wasm_bindgen]
pub fn process_stress_majorization(graph_str: &str, iterations: usize) -> String {
    let mut graph = from_string(graph_str);
    stress_majorization(&mut graph, iterations);
    graph_to_string(&graph)
}

#[wasm_bindgen]
pub fn process_multidimensional_scaling(graph_str: &str, iterations: usize) -> String {
    let mut graph = from_string(graph_str);
    multidimensional_scaling(&mut graph, iterations);
    graph_to_string(&graph)
}
