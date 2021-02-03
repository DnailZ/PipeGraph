
mod builder;
mod edgelist;

struct AdjGraph {
    adj: Vec<edgelist::EdgeList<u64>>
}

impl AdjGraph {
    pub fn new() -> Self {
        AdjGraph{adj: Vec::new()}
    }

    pub fn clear(&mut self) {
        self.clear();
    }
}