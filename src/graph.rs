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

    /// TODO need to add error checking
    pub fn m1r(&mut self) {
        let next_node = *self.in_needle.last().unwrap();
        let prev_node = self.node_below(*self.out_needle.last().unwrap());
        let make_node = self.insert_yarn(next_node, prev_node);
        self.in_needle.push(make_node);
        self.knit();
        //self.yarn_edges.push((self.nodes, new_node));
    }

    fn draw_yarn(&mut self) -> usize {
        self.nodes += 1;
        let new_node = self.nodes;
        self.yarn_edges.push((self.fresh_node, new_node));
        self.fresh_node = new_node;
        new_node
    }

    /// TODO need to add error checking
    fn insert_yarn(&mut self, a: usize, b: usize) -> usize {
        let (position, order) = self.yarn_edges.iter().enumerate().find_map(|(i, &edge)| {
            if edge == (a, b) {
                Some((i, true))
            } else if edge == (b, a) {
                Some((i, false))
            } else {
                None
            }
        }).unwrap();

        let new_node = self.nodes + 1;

        if order {
            self.yarn_edges.push((a, new_node));
            self.yarn_edges.push((new_node, b));
        } else {
            self.yarn_edges.push((b, new_node));
            self.yarn_edges.push((new_node, a));
        }
        self.yarn_edges.swap_remove(position);

        self.nodes += 1;
        new_node
    }

    /// TODO need to add error checking
    fn node_below(&self, node: usize) -> usize {
        let below = self.loop_edges.iter().find(|(_, b)| *b == node).unwrap().0;
        below
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
