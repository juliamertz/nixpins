use anyhow::Result;
use serde::{ser::SerializeStructVariant, Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum Fetcher {
    Github {
        owner: String,
        repo: String,
        rev: String,
        hash: String,
    },
    Gitlab {
        owner: String,
        repo: String,
        rev: String,
        hash: String,
    },
}

fn serialize_simple_fetcher<S>(
    fn_name: &'static str,
    serializer: S,
    owner: &str,
    repo: &str,
    rev: &str,
    hash: &str,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut fetcher = serializer.serialize_struct_variant("Fetcher", 0, fn_name, 4)?;
    fetcher.serialize_field("owner", owner)?;
    fetcher.serialize_field("repo", repo)?;
    fetcher.serialize_field("rev", rev)?;
    fetcher.serialize_field("hash", hash)?;
    fetcher.end()
}

impl Serialize for Fetcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Github {
                owner,
                repo,
                rev,
                hash,
            } => serialize_simple_fetcher(self.fn_name(), serializer, owner, repo, rev, hash),
            Self::Gitlab {
                owner,
                repo,
                rev,
                hash,
            } => serialize_simple_fetcher(self.fn_name(), serializer, owner, repo, rev, hash),
        }
    }
}

impl Fetcher {
    pub fn from_url(url: &str) -> Result<Self> {
        Ok(crate::prefetch::prefetch_url(url)?.into())
    }

    pub fn fn_name(&self) -> &'static str {
        match self {
            Self::Github { .. } => "fetchFromGitHub",
            Self::Gitlab { .. } => "fetchFromGitLab",
        }
    }

    pub fn hash(&self) -> String {
        match self {
            Self::Github { hash, .. } => hash.to_string(),
            Self::Gitlab { hash, .. } => hash.to_string(),
        }
    }

    pub fn rev(&self) -> String {
        match self {
            Self::Github { rev, .. } => rev.to_string(),
            Self::Gitlab { rev, .. } => rev.to_string(),
        }
    }
}
