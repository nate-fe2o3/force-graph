use dioxus::html::circle;
use dioxus::prelude::*;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

#[component]
pub fn MyGraph() -> Element {
    let graph = create_graph();

    // Simple circular layout logic
    let node_count = graph.node_count();
    let radius = 200.0; // Radius of the circle for layout
    let center_x = 250.0;
    let center_y = 250.0;
    let mut node_positions = HashMap::<NodeIndex, (f64, f64)>::new();

    for (i, node_idx) in graph.node_indices().enumerate() {
        if node_count == 0 {
            break;
        } // Avoid division by zero if graph is empty
        let angle = (i as f64 / node_count as f64) * 2.0 * std::f64::consts::PI;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        node_positions.insert(node_idx, (x, y));
    }
    let mut nodes = use_signal(|| node_positions);
    let mut gr = use_signal(|| graph);

    rsx! {
                        circle {
                            cx: "1",
                            cy: "1",
                            r: "15",
                            fill: "#4CAF50",
                            stroke: "#333",
                            stroke_width: "1.5"
                        }
            div {
                style: "width: 500px; height: 500px; border: 1px solid black; margin: auto;", // Container for the SVG
                svg {
                    width: "100%",
                    height: "100%",
                    view_box: "0 0 500 500", // Set a viewbox for consistent scaling

                    // Render edges first so nodes are drawn on top
                for edge in gr.read().edge_references() {
                        if let (Some(pos_source), Some(pos_target)) = (nodes.read().get(&edge.source()), nodes.read().get(&edge.target())) {
                            line {
                                x1: "{pos_source.0}",
                                y1: "{pos_source.1}",
                                x2: "{pos_target.0}",
                                y2: "{pos_target.1}",
                                stroke: "#999",
                                "stroke-width": "2.0"
                            }
                        }
                    }

                    // Render nodes
                for (node_idx, pos) in nodes.read().iter() {
                        if let Some(nt) = gr.read().node_weight(*node_idx) {
                if *nt == NodeType::Value {
                        circle {
                            cx: "{pos.0}",
                            cy: "{pos.1}",
                            r: "15",
                            fill: "#4CAF50",
                            stroke: "#333",
                            stroke_width: "1.5"
                        }

                }
                else {
                        circle {
                            cx: "{pos.0}",
                            cy: "{pos.1}",
                            r: "20",
                            fill: "#2196F3",
                            stroke: "#333",
                            stroke_width: "1.5"
                        }

                }
                        // Optional: Add text labels for nodes
                        // text {
                        //     x: "{pos.0}",
                        //     y: "{pos.1}",
                        //     dy: ".3em", // Offset text slightly for better centering
                        //     text_anchor: "middle",
                        //     fill: "black",
                        //     font_size: "10px",
                        //     format!("{:?}", node_idx.index()) // Display node index as label
                        // }
                    }
                }
            }
        }
    }
}

// Node and Edge types (kept from original file)
#[derive(PartialEq, Eq)]
enum NodeType {
    Value,
    Relationship,
}

enum EdgeDirection {
    Vtr, // Value to Relationship
    Rtv, // Relationship to Value
    Und, // Undirected
}

// Uses example data for now (kept from original file)
fn create_graph() -> Graph<NodeType, EdgeDirection> {
    let mut graph = Graph::<NodeType, EdgeDirection>::new();

    // Create a few more nodes for a slightly more interesting graph
    let v1 = graph.add_node(NodeType::Value);
    let r1 = graph.add_node(NodeType::Relationship);
    let v2 = graph.add_node(NodeType::Value);
    let r2 = graph.add_node(NodeType::Relationship);
    let v3 = graph.add_node(NodeType::Value);

    graph.add_edge(v1, r1, EdgeDirection::Vtr);
    graph.add_edge(r1, v2, EdgeDirection::Rtv);
    graph.add_edge(v2, r2, EdgeDirection::Vtr);
    graph.add_edge(r2, v3, EdgeDirection::Rtv);
    graph.add_edge(v3, r1, EdgeDirection::Und); // Cycle back for fun

    graph
}

