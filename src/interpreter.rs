use crate::nodes::Node;
use std::fmt::Formatter;

pub struct Interpreter {}

pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    #[inline(always)]
    pub fn interpret(&mut self, node: Box<Node>) -> Number {
        self.visit(node)
    }

    fn visit(&mut self, node: Box<Node>) -> Number {
        use Node::*;
        match *node {
            NumberNode(f) => Number::new(f),
            AddNode(node_a, node_b) => {
                Number::new(self.visit(node_a).value + self.visit(node_b).value)
            }
            SubtractNode(node_a, node_b) => {
                Number::new(self.visit(node_a).value - self.visit(node_b).value)
            }
            MultiplyNode(node_a, node_b) => {
                Number::new(self.visit(node_a).value * self.visit(node_b).value)
            }
            DivideNode(node_a, node_b) => {
                Number::new(self.visit(node_a).value / self.visit(node_b).value)
            }
            PlusNode(node) => self.visit(node),
            MinusNode(node) => Number::new(-1.0 * self.visit(node).value),
        }
    }
}
