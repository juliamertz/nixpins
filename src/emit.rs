#[derive(Debug, Default, Clone, Copy)]
pub struct Context {
    depth: usize,
}

impl Context {
    fn indent(mut self) -> Context {
        self.depth += 1;
        self
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Node {
    Identifier(String),
    String(String),
    Lambda(Box<Node>),

    Attrlist(Vec<Node>),
    Attrset(Vec<Node>),
    Attrpath(Vec<String>),

    Assign(Box<Node>, Box<Node>),
    Call(Box<Node>, Box<Node>),
    Let(Vec<Node>),

    Ellipsis,
    Comment(String),
    Raw(String),
}

impl Node {
    pub fn emit(&self, ctx: Context) -> String {
        match self {
            Node::Identifier(text) => text.clone(),
            Node::Lambda(node) => format!("{}:", node.emit(ctx)),
            Node::Attrlist(nodes) => {
                format!(
                    "{{ {nodes} }}",
                    nodes = nodes
                        .iter()
                        .map(|n| n.emit(ctx.indent()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Node::Attrset(nodes) => {
                format!(
                    "{{ {nodes} }}",
                    nodes = nodes
                        .iter()
                        .map(|n| n.emit(ctx.indent()))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Node::Let(nodes) => {
                format!(
                    " let\n{nodes}\nin",
                    nodes = nodes
                        .iter()
                        .map(|n| n.emit(ctx.indent()))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Node::Comment(text) => format!("# {text}"),
            Node::Attrpath(parts) => parts.join("."),
            Node::Assign(left, right) => format!("{} = {};", left.emit(ctx), right.emit(ctx)),
            Node::Call(left, right) => format!("{} {}", left.emit(ctx), right.emit(ctx)),
            Node::Ellipsis => "...".into(),
            Node::Raw(content) => content.trim().to_string(),
            Node::String(content) => format!("\"{content}\""),
        }
    }

    pub fn lambda(left: Node) -> Node {
        Node::Lambda(Box::new(left))
    }

    pub fn assign(left: Node, right: Node) -> Node {
        Node::Assign(Box::new(left), Box::new(right))
    }

    pub fn call(left: Node, right: Node) -> Node {
        Node::Call(Box::new(left), Box::new(right))
    }

    pub fn ident(value: &str) -> Node {
        Node::Identifier(value.to_string())
    }

    pub fn string(value: &str) -> Node {
        Node::String(value.to_string())
    }
}
