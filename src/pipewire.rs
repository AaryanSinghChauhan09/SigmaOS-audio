//! PipeWire-style audio graph model.

#[derive(Debug, Clone)]
pub struct AudioNode {
    pub id: u32,
    pub name: String,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType { Source, Sink, Port }

#[derive(Debug, Clone)]
pub struct Link {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug, Default)]
pub struct AudioGraph {
    pub nodes: Vec<AudioNode>,
    pub links: Vec<Link>,
}

impl AudioGraph {
    pub fn new() -> Self { Self::default() }
    pub fn add_node(&mut self, n: AudioNode) { self.nodes.push(n); }
    pub fn link(&mut self, l: Link) { self.links.push(l); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_graph() {
        let mut g = AudioGraph::new();
        g.add_node(AudioNode { id: 1, name: "mic".into(), node_type: NodeType::Source });
        assert_eq!(g.nodes.len(), 1);
    }
}
