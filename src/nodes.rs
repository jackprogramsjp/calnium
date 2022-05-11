#[derive(Clone, Debug)]
pub enum Node {
    NumberNode(f64),
    AddNode(Box<Node>, Box<Node>),      // (number + number)
    SubtractNode(Box<Node>, Box<Node>), // (number - number)
    MultiplyNode(Box<Node>, Box<Node>), // (number * number)
    DivideNode(Box<Node>, Box<Node>),   // (number / number)
    PlusNode(Box<Node>),                // (+number)
    MinusNode(Box<Node>),               // (-number)
}
