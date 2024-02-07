use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error;

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
    parent_id: u32,
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
            parent_id: 0,
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

    pub fn add_node(&mut self, parent_id: u32, title: String) -> anyhow::Result<()> {
        // 先构造新节点的基础信息，不立即借用parent
        let (n_path, n_level, n_order, p_id) = {
            let parent = self.get_node(parent_id)?.borrow();
            let n_path = format!("{}{}.", parent.path, parent.id);
            let order = parent.children.len();
            (n_path, parent.level + 1, order, parent.id)
        };
        let t_node = Rc::new(RefCell::new(Node {
            id: self.count + 1,
            title: title.clone(),
            is_root: false,
            direction: Direction::default(),
            parent_id: p_id,
            order: n_order,
            path: n_path,
            level: n_level,
            tag: String::new(),
            color: NodeColor::default(),
            children: vec![],
        }));

        self.get_node(parent_id)?
            .borrow_mut()
            .children
            .push(t_node.clone());
        self.node_map.insert(t_node.borrow().id, t_node.clone());
        self.count += 1;
        Ok(())
    }

    pub  fn remove_node(&mut self, node_id: u32) -> anyhow::Result<()> {
        let descendant_node_ids = self.get_descendant_ids(node_id)?;
        for descendant_id in descendant_node_ids {
            self.node_map.remove(&descendant_id);
        }
        self.node_map.remove(&node_id);
        let node = self.get_node(node_id)?.borrow();
        self.get_node(node.parent_id)?.borrow_mut().children.remove(node.order);
        Ok(())
    }

    pub fn move_node(&self, node_id: u32) -> anyhow::Result<()>{

        Ok(())
    }

    pub fn get_node(&self, node_id: u32) -> anyhow::Result<&Rc<RefCell<Node>>> {
        Ok(self
            .node_map
            .get(&node_id)
            .ok_or(error::NodeError::NotFound)?)
    }

    fn get_descendant_ids(&self, node_id: u32) -> anyhow::Result<Vec<u32>> {
        let mut ids:Vec<u32> = Vec::new();
        for child in &self.get_node(node_id)?.borrow().children {
            ids.push(child.borrow().id);
            ids.append(&mut self.get_descendant_ids(child.borrow().id)?);
        }
        Ok(ids)
    }
}
