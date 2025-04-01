use super::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Gitlab {
    owner: String,
    repo: String,
    rev: String,
    hash: String,
}

impl super::Source for Gitlab {
    fn function_name(&self) -> &'static str {
        "fetchFromGitLab"
    }

    fn from_prefetched(pre: Prefetched) -> Self {
        assert_eq!(pre.original.r#type, "gitlab");
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
        todo!()
        // let mut fields = vec![];
        // Node::call(Node::Identifier(self.function_name().to_string()), Node::Attrset(fields))
    }
}
