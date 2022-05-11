use crate::nodes::Node;

pub fn transpile(node: Box<Node>) -> String {
    let mut start = String::from("#include <stdio.h>\n\nint main(void) {\n  printf(\"%lf\\n\", ");
    fn gen(node: Box<Node>) -> String {
        use Node::*;
        match *node {
            NumberNode(f) => {
                let mut res = f.to_string();
                if !res.ends_with('.') {
                    res.push('.');
                    res.push('0');
                }
                res
            }
            AddNode(node_a, node_b) => format!("({} + {})", gen(node_a), gen(node_b)),
            SubtractNode(node_a, node_b) => format!("({} - {})", gen(node_a), gen(node_b)),
            MultiplyNode(node_a, node_b) => format!("({} * {})", gen(node_a), gen(node_b)),
            DivideNode(node_a, node_b) => format!("({} / {})", gen(node_a), gen(node_b)),
            PlusNode(node) => format!("(+{})", gen(node)),
            MinusNode(node) => format!("(-{})", gen(node)),
        }
    }
    start.push_str(gen(node).as_str());
    start.push_str(");\n  return 0;\n}\n");
    start
}
