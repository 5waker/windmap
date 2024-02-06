use std::{cell::RefCell, collections::HashMap, default, ops::Add, rc::Rc};

pub struct Tree {
    node_map: HashMap<u32, Rc<RefCell<Node>>>,
    pub root: Rc<RefCell<Node>>,
    count: u32,
}

pub struct Node {
    pub id: u32,
    pub title: String,
    pub is_root: bool,
    pub direction: Direction,
    order: usize,
    path: String,
    level: u16,
    tag: String,
    color: NodeColor,
    pub children: Vec<Rc<RefCell<Node>>>,
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

impl Tree {
    pub fn new(title: String) -> Self {
        let t_root: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
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
        }));
        let mut t_node_map = HashMap::new();
        t_node_map.insert(1, t_root.clone());
        Self {
            node_map: t_node_map,
            root: t_root,
            count: 1,
        }
    }

    pub fn add_node(&mut self, parent_id: u32, title: String) {
        self.count += 1;
        // 先构造新节点的基础信息，不立即借用parent
        let (n_path, parent_level, n_order) = {
            let parent = self
                .node_map
                .get(&parent_id)
                .expect("Parent not found")
                .borrow();
            let n_path = format!("{}{}.", parent.path, parent.id);
            let order = parent.children.len();
            (n_path, parent.level + 1, order)
        };
        let t_node = Rc::new(RefCell::new(Node {
            id: self.count,
            title: title.clone(),
            is_root: false,
            direction: Direction::default(),
            order: n_order,
            path: n_path,
            level: parent_level + 1,
            tag: String::new(),
            color: NodeColor::default(),
            children: vec![],
        }));
        self.node_map.insert(t_node.borrow().id, t_node.clone());
    }

    pub fn get_node(&self, node_id: u32) -> &Rc<RefCell<Node>> {
        self.node_map.get(&node_id).unwrap()
    }
}
