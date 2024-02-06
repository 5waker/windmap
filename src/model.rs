use std::{collections::HashMap, default};

pub struct Tree<'a> {
    node_map: HashMap<u32, &'a Node>,
    pub root: Node,
    count: u32,
}

pub struct Node {
    pub id: u32,
    pub title: String,
    pub is_root: bool,
    pub direction: Direction,
    order: u32,
    path: String,
    level: u16,
    tag: String,
    color: NodeColor,
    pub children: Vec<Node>,
}

#[derive(Default)]
pub enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Default)]
pub enum NodeColor {
    White,
    Green,
    #[default]
    Gray,
    Red,
    Blue,
}

impl<'a> Tree<'a> {
    pub fn new(title: String) -> Self {
        let root = Node {
            id: 1,
            title: title.clone(),
            is_root: true,
            direction: Direction::default(),
            order: 0,
            path: String::from("."),
            level: 0,
            tag: String::new(),
            color: NodeColor::default(),
            children: vec![],
        };
        let mut node_map = HashMap::new();
        node_map.insert(root.id, &root);
        Self {
            node_map: node_map,
            root: root,
            count: 1,
        }
    }
}
