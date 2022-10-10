use std::collections::VecDeque;

use crate::prelude::*;

pub struct Pathfinder {
    map: Map,
}

#[derive(Clone)]
struct Node {
    g_cost: f32,
    h_cost: f32,
    parent_node: Option<Box<Node>>,
    pos: Vec2,
}

impl Node {
    pub fn f_cost(&self) -> f32 {
        self.g_cost + self.h_cost
    }
}

impl Pathfinder {
    pub fn new(map: Map) -> Self {
        Pathfinder { map }
    }

    pub fn a_star(&self, start: Vec2, end: Vec2) -> Option<VecDeque<Vec2>> {
        let end_tile = idx_to_vec2(vec2_to_idx(end) as i32);
        let start_tile = idx_to_vec2(vec2_to_idx(start) as i32);

        if !self.map.tiles[vec2_to_idx(end_tile)].is_traversable() {
            return None;
        }
        let destination_vec = end_tile - start_tile;
        println!(
            "{}, {} / {}, {}",
            start_tile.x, start_tile.y, end_tile.x, end_tile.y
        );
        let mut path: Vec<Vec2> = Vec::new();
        let mut open_nodes: Vec<Node> = vec![Node {
            g_cost: 0.0,
            h_cost: destination_vec.length().ceil(),
            parent_node: None,
            pos: start_tile,
        }];
        let mut closed_nodes: Vec<Node> = vec![];

        let mut current_node: Node;
        while open_nodes.len() > 0 {
            current_node = open_nodes[0].clone();
            for node in open_nodes.iter() {
                if node.pos == current_node.pos {
                    continue;
                }

                if node.f_cost() < current_node.f_cost()
                    || node.f_cost() == current_node.f_cost() && node.h_cost < current_node.h_cost
                {
                    current_node = node.clone();
                }
            }

            // println!("{}, {}", current_node.pos.x, current_node.pos.y);
            closed_nodes.push(current_node.clone());

            let mut idx_to_remove: Option<usize> = None;
            for (idx, node) in open_nodes.iter().enumerate() {
                if node.pos == current_node.pos {
                    idx_to_remove = Some(idx);
                }
            }
            open_nodes.remove(idx_to_remove.unwrap());

            if current_node.pos.x == end_tile.x && current_node.pos.y == end_tile.y {
                // Found exit
                path = self.retrace_path(current_node);
                path.reverse();
                break;
            }
            let mut neighbours =
                self.evaluate_traversable_node_neighbours(&current_node, start_tile, end_tile);
            for n in neighbours.iter_mut() {
                if vec_contains_node(&closed_nodes, n) {
                    continue;
                }

                let current_to_neighbour_vec = n.pos - current_node.pos;
                let new_cost_to_neighbour =
                    current_node.g_cost + current_to_neighbour_vec.length().ceil();
                if new_cost_to_neighbour < n.g_cost || !vec_contains_node(&open_nodes, n) {
                    n.g_cost = new_cost_to_neighbour.ceil();
                    n.h_cost = (end_tile - n.pos).length().ceil();

                    n.parent_node = Some(Box::new(current_node.clone()));

                    if !vec_contains_node(&open_nodes, n) {
                        open_nodes.push(n.clone());
                    }
                }
            }
        }

        Some(VecDeque::from(path))
    }

    fn retrace_path(&self, end: Node) -> Vec<Vec2> {
        let mut path: Vec<Vec2> = Vec::new();

        let mut current_node = &end;
        loop {
            if let Some(parent) = &current_node.parent_node {
                path.push(current_node.pos);
                current_node = &parent;
            } else {
                break;
            }
        }
        path
    }

    fn evaluate_traversable_node_neighbours(
        &self,
        node: &Node,
        start_pos: Vec2,
        target_pos: Vec2,
    ) -> Vec<Node> {
        let mut neigbours: Vec<Node> = Vec::new();
        let diff_vectors = vec![
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(-1.0, -1.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(-1.0, 1.0),
        ];

        for diff in diff_vectors.iter() {
            let current_neigbour_pos = node.pos + *diff * TILE_SIZE as f32;
            if !self.map.in_bounds(current_neigbour_pos) {
                continue;
            }
            let neighbour_idx = vec2_to_idx(current_neigbour_pos);
            if self.map.tiles[neighbour_idx].is_traversable() {
                let destination_vec: Vec2 = target_pos - current_neigbour_pos;
                let start_vec: Vec2 = start_pos - current_neigbour_pos;
                neigbours.push(Node {
                    g_cost: start_vec.length().ceil(),
                    h_cost: destination_vec.length().ceil(),
                    parent_node: Some(Box::new(node.clone())),
                    pos: current_neigbour_pos,
                });
            }
        }

        neigbours
    }
}

fn vec_contains_node(nodes: &Vec<Node>, node: &Node) -> bool {
    for n in nodes.iter() {
        if n.pos == node.pos {
            return true;
        }
    }
    return false;
}
