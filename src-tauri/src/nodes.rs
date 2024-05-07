use std::collections::HashMap;

pub struct Node {
    pub value: String,

    parent: Option<i32>,
    alternatives: Vec<i32>,
    children: Vec<i32>,

    pub checksum: String,
}

pub struct ArenaTree {
    pub nodes_map: HashMap<i32, Node>,

    last_generated_id: i32,
}

impl ArenaTree {
    pub fn new() -> Self {
        Self {
            nodes_map: HashMap::new(),
            last_generated_id: 0,
        }
    }
    
    /// Generates new id and returns it.
    fn generate_id(&mut self) -> i32 {
        self.last_generated_id += 1;
        self.last_generated_id
    }

    /// Returns node of this [`ArenaTree`] by id.
    pub fn get_node_by_id(&self, id: i32) -> Option<&Node> {
        self.nodes_map.get(&id)
    }

    /// Returns root node of this [`ArenaTree`].
    pub fn get_root_node(&self) -> Option<&Node> {
        self.nodes_map.get(&0)
    }

    /// Adds new node.
    pub fn add_node_with_parent_node(&mut self, mut parent_node: Node, node: Node) -> () {
        let id = self.generate_id();

        self.nodes_map.insert(id, node);
        parent_node.children.push(id);   
    }

    /// Adds new node with parent id.
    pub fn add_node_with_parent_id(&mut self, parent_id: i32, node: Node) -> () {
        let id = self.generate_id();
        self.nodes_map.insert(id, node);
        self.nodes_map.get_mut(&parent_id).unwrap().children.push(id);
    }
}
