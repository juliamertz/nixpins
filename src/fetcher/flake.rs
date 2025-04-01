use super::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Flake {
    pub url: crate::url::Url,
    pub rev: String,
    pub hash: String,
}

impl Source for Flake {
    fn function_name(&self) -> &'static str {
        "fetchFlake"
    }

    fn from_prefetched(pre: Prefetched) -> Self {
        Self {
            url: crate::url::Url::try_from("github:juliamertz/nixpins".to_string()).unwrap(),
            rev: pre.locked.rev,
            hash: pre.hash,
        }
    }

    fn hash(&self) -> &str {
        &self.hash
    }

    fn version(&self) -> &str {
        &self.rev
    }

    fn node(&self) -> Node {
        Node::call(
            Node::Identifier(self.function_name().to_string()),
            Node::Attrset(vec![
                Node::assign(
                    Node::ident("url"),
                    Node::string(&self.url.fmt_clean().unwrap()),
                ),
                Node::assign(Node::ident("rev"), Node::string(&self.rev)),
                Node::assign(Node::ident("hash"), Node::string(&self.hash)),
            ]),
        )
    }
}
