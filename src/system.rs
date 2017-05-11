use node::Node;

#[derive(Clone, Debug)]
pub struct System {
    nodes: Vec<Node>,
}

impl System {
    pub fn new() -> Result<System, &'static str> {
        let system: System = Default::default();
        system.init()
    }

    fn init(mut self) -> Result<Self, &'static str> {
        // TODO: popolate Nodes
        for i in 0..4096 {
            match Node::new(i) {
                Ok(n) => {
                    self.nodes.push(n);
                }
                Err(_) => {
                    break;
                }
            }
        }

        Ok(self)
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }
}

impl Default for System {
    fn default() -> System {
        System { nodes: Vec::new() }
    }
}
