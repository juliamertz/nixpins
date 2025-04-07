#[derive(Debug, Clone, Copy)]
pub struct Context {
    depth: usize,
    indent_size: usize,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            depth: 1,
            indent_size: 2,
        }
    }
}

impl Context {
    fn indented(mut self) -> Context {
        self.depth += 1;
        self
    }

    fn dedented(mut self) -> Context {
        self.depth -= 1;
        self
    }

    fn indent_str(&self) -> String {
        " ".repeat(self.depth * self.indent_size)
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
                        .map(|n| n.emit(ctx.indented()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Node::Attrset(nodes) => {
                format!(
                    "{{\n{nodes}\n{indent}}}",
                    indent = ctx.dedented().indent_str(),
                    nodes = nodes
                        .iter()
                        .map(|n| format!(
                            "{indent}{text}",
                            indent = ctx.indent_str(),
                            text = n.emit(ctx.indented())
                        ))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            }
            Node::Let(nodes) => {
                let text = if nodes.len() == 1 && nodes.first().unwrap().emit(ctx).len() <= 60 {
                    format!(" {} ", nodes.first().unwrap().emit(ctx))
                } else {
                    format!(
                        "\n{}\n",
                        nodes
                            .iter()
                            .map(|n| format!(
                                "{indent}{text}",
                                indent = ctx.indent_str(),
                                text = n.emit(ctx.indented())
                            ))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                };

                format!("let{text}in")
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
