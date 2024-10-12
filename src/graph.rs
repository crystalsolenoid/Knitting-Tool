use std::mem;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum KnitError {
    NeedleEmpty,
}

impl Display for KnitError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            KnitError::NeedleEmpty => write!(f, "Needle is empty"),
        }
    }
}

#[derive(Debug)]
pub struct DirectedKnittingGraph {
    pub yarn_edges: Vec<(usize, usize)>,
    pub loop_edges: Vec<(usize, usize)>,
    pub nodes: usize,
    pub in_needle: Vec<usize>,
    pub out_needle: Vec<usize>,
    pub fresh_node: usize,
}

impl DirectedKnittingGraph {
    pub fn new() -> Self {
        Self{
            yarn_edges: vec![],
            loop_edges: vec![],
            nodes: 0,
            in_needle: vec![],
            out_needle: vec![],
            fresh_node: 0,
        }
    }

    pub fn cast_on(&mut self, n: usize) {
        // Can I find a way to use draw_yarn() for this?
        let nodes = self.nodes;
        self.yarn_edges.extend(
            (nodes..nodes + n).zip(nodes+1..nodes + n + 1)
        );
        self.out_needle.extend(nodes + 1..nodes + n + 1);
        self.nodes += n;
        self.fresh_node = self.nodes;
    }

    pub fn turn(&mut self) {
        mem::swap(&mut self.in_needle, &mut self.out_needle);
    }

    pub fn knit(&mut self) -> Result<(), KnitError> {
        let on_needle = self.in_needle.pop();
        if let Some(on_needle) = on_needle {
            let new_node = self.draw_yarn();
            self.loop_edges.push((on_needle, new_node));
            self.out_needle.push(new_node);
            Ok(())
        } else {
            Err(KnitError::NeedleEmpty)
        }
    }

    pub fn k2tog(&mut self) -> Result<(), KnitError> {
        let on_needle = [self.in_needle.pop(), self.in_needle.pop()];
        let on_needle: Option<Vec<_>> = on_needle.into_iter().collect();
        if let Some(on_needle) = on_needle {
            let new_node = self.draw_yarn();
            for needle_node in on_needle {
                self.loop_edges.push((needle_node, new_node));
            }
            self.out_needle.push(new_node);
            Ok(())
        } else {
            Err(KnitError::NeedleEmpty)
        }
    }

    pub fn yo(&mut self) {
        let new_node = self.draw_yarn();
        self.out_needle.push(new_node);
    }

    fn draw_yarn(&mut self) -> usize {
        self.nodes += 1;
        let new_node = self.nodes;
        self.yarn_edges.push((self.fresh_node, new_node));
        self.fresh_node = new_node;
        new_node
    }

    pub fn dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("Digraph {\n");
        for (a, b) in &self.yarn_edges {
            dot += &format!("{} -> {} [color=blue]\n", a, b);
        }
        for (a, b) in &self.loop_edges {
            dot += &format!("{} -> {} [color=red]\n", a, b);
        }
        dot.push_str("}\n");
        dot
    }
}
