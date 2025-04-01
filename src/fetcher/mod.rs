pub mod flake;
pub mod github;
pub mod gitlab;

use std::fmt::Display;

use crate::emit::Node;
pub(super) use crate::{prefetch::Prefetched, url::Url};
pub(super) use anyhow::Result;
pub(super) use serde::Deserialize;

pub trait Source {
    fn function_name(&self) -> &'static str;
    fn node(&self) -> Node;
    fn from_prefetched(pre: Prefetched) -> Self;
    fn hash(&self) -> &str;
    fn version(&self) -> &str;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Fetcher {
    Github(github::Github),
    Gitlab(gitlab::Gitlab),
    Flake(flake::Flake),
}

impl Fetcher {
    pub fn from_url(url: impl Display, flake: bool) -> Result<Self> {
        let url = Url::try_from(url.to_string())?;
        let pre = crate::prefetch::prefetch_url(&url)?;
        if flake {
            Ok(Fetcher::Flake(flake::Flake {
                url,
                rev: pre.locked.rev,
                hash: pre.hash,
            }))
        } else {
            Ok(Fetcher::from_prefetched(pre))
        }
    }
}

impl Source for Fetcher {
    fn function_name(&self) -> &'static str {
        match self {
            Self::Github(v) => v.function_name(),
            Self::Gitlab(v) => v.function_name(),
            Self::Flake(v) => v.function_name(),
        }
    }

    fn from_prefetched(pre: Prefetched) -> Self {
        match pre.original.r#type.as_str() {
            "github" => Fetcher::Github(github::Github::from_prefetched(pre)),
            "gitlab" => Fetcher::Gitlab(gitlab::Gitlab::from_prefetched(pre)),
            _ => unimplemented!(),
        }
    }

    fn hash(&self) -> &str {
        match self {
            Self::Github(field) => field.hash(),
            Self::Gitlab(field) => field.hash(),
            Self::Flake(field) => field.hash(),
        }
    }

    fn version(&self) -> &str {
        match self {
            Self::Github(field) => field.version(),
            Self::Gitlab(field) => field.version(),
            Self::Flake(field) => field.version(),
        }
    }

    fn node(&self) -> Node {
        match self {
            Self::Github(field) => field.node(),
            Self::Gitlab(field) => field.node(),
            Self::Flake(field) => field.node(),
        }
    }
}
