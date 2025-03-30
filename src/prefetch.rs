use anyhow::Result;
use serde::Deserialize;
use std::{path::PathBuf, process::Command};

#[derive(Debug, Deserialize)]
pub struct Locked {
    #[serde(rename = "lastModified")]
    pub last_modified: i64,
    #[serde(rename = "narHash")]
    pub nar_hash: String,
    pub owner: String,
    pub repo: String,
    pub rev: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Original {
    pub owner: String,
    pub repo: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Prefetched {
    pub hash: String,
    pub locked: Locked,
    pub original: Original,
    #[serde(rename = "storePath")]
    pub store_path: PathBuf,
}

impl From<Prefetched> for crate::fetcher::Fetcher {
    fn from(pre: Prefetched) -> Self {
        match pre.original.r#type.as_str() {
            "github" => Self::Github {
                owner: pre.original.owner,
                repo: pre.original.repo,
                rev: pre.locked.rev,
                hash: pre.hash,
            },
            "gitlab" => Self::Gitlab {
                owner: pre.original.owner,
                repo: pre.original.repo,
                rev: pre.locked.rev,
                hash: pre.hash,
            },
            _ => unimplemented!(),
        }
    }
}

pub fn prefetch_url(url: &str) -> Result<Prefetched> {
    let output = Command::new("nix")
        .args([
            "flake",
            "prefetch",
            "--refresh",
            "--extra-experimental-features",
            "'nix-command flakes'",
            "--json",
            url,
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(serde_json::from_str(stdout.to_string().as_ref())?)
}
