use dioxus::html::circle;
use dioxus::html::u::transform;
use dioxus::prelude::*;
use glam::{DVec2, Vec2};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

const NODE_RADIUS: f64 = 10.;

#[derive(Props, Clone, PartialEq)]
struct EdgeProps {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    direction: EdgeDirection,
}

#[component]
pub fn MyGraph() -> Element {
    let graph_data = create_graph();

    let node_count = graph_data.node_count();
    let radius = 200.0;
    let center_x = 250.0;
    let center_y = 250.0;
    let mut initial_node_positions = HashMap::<NodeIndex, (f64, f64)>::new();

    for (i, node_idx) in graph_data.node_indices().enumerate() {
        if node_count == 0 {
            break;
        }
        let angle = (i as f64 / node_count as f64) * 2.0 * std::f64::consts::PI;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        initial_node_positions.insert(node_idx, (x, y));
    }
    let nodes = use_signal(|| initial_node_positions);
    let graph = use_signal(|| graph_data);

    rsx! {
        div {
            style: "width: 500px; height: 500px; border: 1px solid black; margin: auto;",
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 0 500 500",
                defs {
                    // Arrowhead marker definition
                    marker {
                        id: "arrowhead",
                        view_box: "0 0 10 10",
                        ref_x: "8",
                        ref_y: "5",
                        marker_width: "6",
                        marker_height: "6",
                        orient: "auto-start-reverse",
                        path {
                            d: "M 0 0 L 10 5 L 0 10 z", // triangle
                            fill: "#2196F3"
                        }
                    }
                }

                for edge_ref in graph.read().edge_references() {
                    if let (Some(pos_source), Some(pos_target)) = (nodes.read().get(&edge_ref.source()), nodes.read().get(&edge_ref.target())) {
                        Edge {
                            x1: pos_source.0,
                            y1: pos_source.1,
                            x2: pos_target.0,
                            y2: pos_target.1,
                            direction: *edge_ref.weight()
                        }
                    }
                }

                for (node_idx, pos) in nodes.read().iter() {
                    if let Some(nt) = graph.read().node_weight(*node_idx) {
                        if *nt == NodeType::Value {
                            circle {
                                cx: pos.0,
                                cy: pos.1,
                                r: NODE_RADIUS,
                                fill: "#4CAF50",
                                stroke: "#333",
                                stroke_width: 1.5
                            }
                        } else {
                            circle {
                                cx: pos.0,
                                cy: pos.1,
                                r: NODE_RADIUS,
                                fill: "#2196F3",
                                stroke: "#333",
                                stroke_width: 1.5
                            }
                        }
                        text {
                            x: pos.0,
                            y: pos.1,
                            dy: ".3em",
                            text_anchor: "middle",
                            fill: "black",
                            font_size: "10px",
                            {format!("{:?}", node_idx.index())}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Edge(props: EdgeProps) -> Element {
    let start_point = DVec2::new(props.x1, props.y1);
    let end_point = DVec2::new(props.x2, props.y2);
    let len_offset = NODE_RADIUS + 15.;
    let diff = end_point - start_point;
    let length = diff.length() - (NODE_RADIUS + 15.);
    let angle_rad = diff.to_angle();
    let angle_deg = angle_rad.to_degrees();
    let offset = DVec2::from_angle(angle_rad) * len_offset / 2.;

    let path_d = format!("M0,0 L{},0", length);

    let marker_start_attr = match props.direction {
        EdgeDirection::Rtv | EdgeDirection::Bi => Some("url(#arrowhead)"),
        _ => None,
    };

    let marker_end_attr = match props.direction {
        EdgeDirection::Vtr | EdgeDirection::Bi => Some("url(#arrowhead)"),
        _ => None,
    };

    rsx! {
        path {
            stroke: "#2196F3",
            stroke_width: 2,
            d: path_d,
            transform: format!("translate({}, {}) rotate({})", props.x1 + offset.x, props.y1 + offset.y, angle_deg),
            marker_start: marker_start_attr,
            marker_end: marker_end_attr,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Added derives
enum NodeType {
    Value,
    Relationship,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Added derives
enum EdgeDirection {
    Vtr, // Value to Relationship
    Rtv, // Relationship to Value
    Und, // Undirected
    Bi,
}

fn create_graph() -> Graph<NodeType, EdgeDirection> {
    let mut graph = Graph::<NodeType, EdgeDirection>::new();
    let v1 = graph.add_node(NodeType::Value);
    let r1 = graph.add_node(NodeType::Relationship);
    let v2 = graph.add_node(NodeType::Value);
    let r2 = graph.add_node(NodeType::Relationship);
    let v3 = graph.add_node(NodeType::Value);
    let r3 = graph.add_node(NodeType::Relationship);

    graph.add_edge(v1, r1, EdgeDirection::Vtr); // Arrow to r1
    graph.add_edge(v2, r2, EdgeDirection::Rtv); // Arrow to v2
    graph.add_edge(v3, r3, EdgeDirection::Und); // No arrows
    graph.add_edge(r1, v2, EdgeDirection::Bi); // Arrows on both ends
    graph.add_edge(r2, v3, EdgeDirection::Vtr); // Example for another VTR

    graph
}
