use super::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Github {
    owner: String,
    repo: String,
    rev: String,
    hash: String,
}

impl super::Source for Github {
    fn function_name(&self) -> &'static str {
        "fetchFromGitHub"
    }

    fn from_prefetched(pre: Prefetched) -> Self {
        assert_eq!(pre.original.r#type, "github");
        Self {
            owner: pre.original.owner,
            repo: pre.original.repo,
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
                Node::assign(Node::ident("owner"), Node::string(&self.owner)),
                Node::assign(Node::ident("repo"), Node::string(&self.repo)),
                Node::assign(Node::ident("rev"), Node::string(&self.rev)),
                Node::assign(Node::ident("hash"), Node::string(&self.hash)),
            ]),
        )
    }
}
