use anyhow::Result;
use serde::Deserialize;
use std::{path::PathBuf, process::Command};

use crate::{
    fetcher::{self, Fetcher, Source},
    url::Url,
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Locked {
    pub last_modified: i64,
    pub nar_hash: Option<String>,
    pub owner: String,
    pub repo: String,
    pub rev: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub owner: String,
    pub repo: String,
    pub r#type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prefetched {
    pub hash: String,
    pub locked: Locked,
    pub original: Original,
    pub store_path: PathBuf,
}

impl From<Prefetched> for Fetcher {
    fn from(pre: Prefetched) -> Self {
        match pre.original.r#type.as_str() {
            "github" => Fetcher::Github(fetcher::github::Github::from_prefetched(pre)),
            "gitlab" => Fetcher::Gitlab(fetcher::gitlab::Gitlab::from_prefetched(pre)),
            _ => unimplemented!(),
        }
    }
}

pub fn prefetch_url(url: &Url) -> Result<Prefetched> {
    let output = Command::new("nix")
        .args([
            "flake",
            "prefetch",
            "--refresh",
            "--extra-experimental-features",
            "'nix-command flakes'",
            "--json",
            &url.fmt()?,
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout = stdout.to_string();

    println!("{}", &stdout);

    Ok(serde_json::from_str(stdout.as_ref())?)
}
