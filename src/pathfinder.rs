use crate::prelude::*;

struct Pathfinder {
    map: Map,
}

#[derive(Clone)]
struct Node{
    g_cost: i32,
    h_cost: i32,
    parent_tile: Option<Box<Node>>,
    pos: Vec2,
}

impl Node {
    pub fn f_cost(&self) -> i32 {
        self.g_cost + self.h_cost
    }
}

impl Pathfinder {
    pub fn new(map: Map) -> Self {
        Pathfinder {
            map,
        }
    }


    pub fn a_star(start: Vec2, end: Vec2) {
        let mut path: Vec<Vec2> = vec![];
        let mut open_nodes: Vec<Node> = vec![Node{ g_cost: 0, h_cost: 0, parent_tile: None, pos: start}]; 
        let mut closed_nodes: Vec<Node> = vec![];

        let mut current_node_idx = 0;
        loop {
            let current_node: Node = open_nodes[current_node_idx].clone();
            for (idx, node) in open_nodes.iter().enumerate(){
                if node.f_cost() < open_nodes[current_node_idx].f_cost(){
                    current_node_idx = idx;
                }
            }

            closed_nodes.push(open_nodes[current_node_idx].clone());
            open_nodes.remove(current_node_idx); 

            if current_node.pos.x == end.x && current_node.pos.y == end.y {
                // Found exit
                break;
            }


        }
    }
}
