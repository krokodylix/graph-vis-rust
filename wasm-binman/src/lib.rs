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
#[derive(Debug,Clone, PartialEq)]
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




#[cfg(test)]
pub mod tests {
    use super::*;

    use wasm_bindgen_test::*;


    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }
   

    #[wasm_bindgen_test]
    fn test_process_fruchterman_reingold() {
        let graph_str = "0-1,1-2,3-4,2-3,2-4,5-9,1-5,2-6"; 
        let iterations = 10;
        let gravity = 1.0;
        let result = process_fruchterman_reingold(graph_str, iterations, gravity);
        let start = result.find("edges: ").unwrap_or(0);
        let expected_result = "edges: ".to_owned() + graph_str + ","; 
        assert_eq!(&result[start..], expected_result);
    }

    #[wasm_bindgen_test]
    fn test_process_stress_majorization() {
        let graph_str = "0-1,1-2,3-4,2-3,2-4,4-5,5-6,6-7,7-8,8-9,9-10,10-11,11-12,12-13,13-14,14-15";
        let iterations = 20;

        let result = process_stress_majorization(graph_str, iterations);

        let start = result.find("edges: ").unwrap_or(0);
        let expected_result = "edges: ".to_owned() + graph_str + ","; 
        assert_eq!(&result[start..], expected_result);

    }


    #[wasm_bindgen_test]
    fn test_process_random() {
        let graph_str = "0-1,1-2,3-4,2-3,2-4,5-9,1-5,2-6,7-8,8-1,10-11,9-11"; 
        let result = process_random(graph_str);
        let start = result.find("edges: ").unwrap_or(0);
        let expected_result = "edges: ".to_owned() + graph_str + ","; 
        assert_eq!(&result[start..], expected_result);
    }


    #[wasm_bindgen_test]
    fn test_new_graph() {
        let num_nodes = 5;
        let edges = vec![
            Edge { source: 0, target: 1 },
            Edge { source: 1, target: 2 },
            Edge { source: 2, target: 3 },
            Edge { source: 3, target: 4 },
        ];

        let graph = new_graph(num_nodes, edges.clone());

        assert_eq!(graph.nodes.len(), num_nodes);
        assert_eq!(graph.edges, edges);

        for node in &graph.nodes {
            assert!(node.position.x > 0.0 && node.position.x < 100.0);
            assert!(node.position.y > 0.0 && node.position.y < 100.0);
        }
    }

    #[wasm_bindgen_test]
    fn test_process_circular() {
        let graph_str = "0-1,1-2,2-3,3-4,4-0,1-5,5-6,6-7,6-8,6-9,6-10";
    

        let result = process_circular(graph_str);

        // Parse the result and check the coordinates
        let items: Vec<&str> = result.split(';').collect();
        for item in items {
            if item.starts_with("nodes:") {
                let node_str = &item[6..];
                let parts: Vec<&str> = node_str.split(',').collect();
                assert_eq!(parts.len(), 2, "Unexpected parts length: {:?}, node_str: {}", parts, node_str);
                let x: f64 = parts[0].trim().parse().unwrap();
                assert!(x > 0.0 && x <= 100.0, "x coordinate is not in the expected range: {}", x);
                let y: f64 = parts[1].trim().parse().unwrap();
                assert!(y > 0.0 && y <= 100.0, "y coordinate is not in the expected range: {}", y);
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_process_force_atlas2() {
        let graph_str = "0-1,1-2,2-3,3-4,4-0";
        let iterations = 10;
        let gravity = 1.0;
        let scaling_ratio = 1.0;

        let result = process_force_atlas2(graph_str, iterations, gravity, scaling_ratio);

        // Parse the result and check the coordinates
        let items: Vec<&str> = result.split(';').collect();
        for item in items {
            if item.starts_with("nodes:") {
                let node_str = &item[6..];
                let parts: Vec<&str> = node_str.split(',').collect();
                assert_eq!(parts.len(), 2, "Unexpected parts length: {:?}, node_str: {}", parts, node_str);
                println!("parts[0]: {}", parts[0]); // print the value of parts[0]
                let x: f64 = parts[0].trim().parse().unwrap();
                assert!(x > 0.0 && x <= 100.0, "x coordinate is not in the expected range: {}", x);
                let y: f64 = parts[1].trim().parse().unwrap();
                assert!(y > 0.0 && y <= 100.0, "y coordinate is not in the expected range: {}", y);
            }
        }
    }

    #[test]
    fn test_graph_to_string() {
        let nodes = vec![
            Node { position: Point { x: 1.0, y: 2.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 3.0, y: 4.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 5.0, y: 6.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 7.0, y: 8.0 }, disp: Point { x: 0.0, y: 0.0 } },
        ];
        let edges = vec![
            Edge { source: 0, target: 1 },
            Edge { source: 2, target: 3 },
        ];
        let graph = Graph { nodes, edges };

        let graph_str = graph_to_string(&graph);

        assert_eq!(graph_str, "nodes: 1,2;3,4;5,6;7,8;edges: 0-1,2-3,");
    }

    #[wasm_bindgen_test]
    fn test_multidimensional_scaling() {
        let num_nodes = 5;
        let edges = vec![
            Edge { source: 0, target: 1 },
            Edge { source: 1, target: 2 },
            Edge { source: 2, target: 3 },
            Edge { source: 3, target: 4 },
            Edge { source: 4, target: 0 },
        ];
        let mut graph = new_graph(num_nodes, edges.clone());

        multidimensional_scaling(&mut graph, 10);

        // Check that the graph has the correct number of nodes and edges
        assert_eq!(graph.nodes.len(), num_nodes);
        assert_eq!(graph.edges, edges);

        // Check that all nodes have a position with x and y between 0 and 100
        for node in &graph.nodes {
            assert!(node.position.x > 0.0 && node.position.x < 100.0);
            assert!(node.position.y > 0.0 && node.position.y < 100.0);
        }
    }


    #[test]
    fn test_circular_layout() {
        let nodes = vec![
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
        ];
        let edges = vec![];
        let mut graph = Graph { nodes, edges };

        circular_layout(&mut graph);

        let expected_positions = vec![
            Point { x: 100.0, y: 50.0 },
            Point { x: 50.0, y: 100.0 },
            Point { x: 0.0, y: 50.0 },
            Point { x: 50.0, y: 0.0 },
        ];

        for (node, expected_position) in graph.nodes.iter().zip(expected_positions.iter()) {
            assert!((node.position.x - expected_position.x).abs() < 1e-6);
            assert!((node.position.y - expected_position.y).abs() < 1e-6);
        }
    }


  
    
    #[wasm_bindgen_test]
    fn test_random_layout() {
        // Create a graph with some nodes
    let mut graph = Graph {
        nodes: vec![
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
            Node { position: Point { x: 0.0, y: 0.0 }, disp: Point { x: 0.0, y: 0.0 } },
        ],
        edges: vec![],
    };

        // Apply the random layout
        random_layout(&mut graph);

        // Check that all nodes have a position with x and y between 0 and 100
        for node in &graph.nodes {
            assert!(node.position.x >= 0.0 && node.position.x <= 100.0);
            assert!(node.position.y >= 0.0 && node.position.y <= 100.0);
        }
    }
  
}
