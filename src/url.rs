use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt::Write;

/// Url structure for fetchers
///
/// For example:
/// `https://github.com/nixos/nixpkgs/nixos-unstable`
/// `github:nixos/nixpkgs/nixos-unstable`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Url {
    pub provider: Provider,
    pub owner: String,
    pub repo: String,
    pub tag: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Provider {
    Github,
    Gitlab,
}

const SHORTHAND_PROVIDERS: &[&str] = &["github", "gitlab"];

fn parse_provider(url: &str) -> Result<(Provider, String)> {
    if let Some(prefix) = url.split(":").next() {
        if SHORTHAND_PROVIDERS.contains(&prefix) {
            let parts = url.split(":").collect::<Vec<_>>();
            match parts.as_slice() {
                ["github", rest] => return Ok((Provider::Github, rest.to_string())),
                ["gitlab", rest] => return Ok((Provider::Gitlab, rest.to_string())),
                _ => unreachable!(),
            }
        }
    }

    if let Some(suffix) = url.strip_prefix("https://") {
        let mut parts = suffix.split("/");
        return Ok((
            match parts.next().context("expected domain name")? {
                "github.com" => Provider::Github,
                "gitlab.com" => Provider::Gitlab,
                domain => anyhow::bail!("unkown provider {domain}"),
            },
            parts.collect::<Vec<_>>().join("/"),
        ));
    }

    anyhow::bail!("unable to parse provider from url: {url}")
}

impl TryFrom<String> for Url {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (provider, rest) = parse_provider(&value)?;

        let mut parts = rest.split("/");
        let owner = parts
            .next()
            .context("expected repo owner name")?
            .to_string();
        let repo = parts.next().context("expected repo name")?.to_string();

        Ok(Url {
            provider,
            owner,
            repo,
            tag: parts.next().map(|v| v.to_string()),
        })
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        Url::try_from(input).map_err(serde::de::Error::custom)
    }
}

impl Url {
    /// Format url without and revision
    pub fn fmt_clean(&self) -> Result<String> {
        let mut f = String::new();
        f.write_str(match self.provider {
            Provider::Github => "github",
            Provider::Gitlab => "gitlab",
        })?;
        f.write_str(":")?;
        f.write_str(&self.owner)?;
        f.write_str("/")?;
        f.write_str(&self.repo)?;
        Ok(f)
    }

    pub fn fmt(&self) -> Result<String> {
        let mut f = String::new();
        f.write_str(match self.provider {
            Provider::Github => "github",
            Provider::Gitlab => "gitlab",
        })?;
        f.write_str(":")?;
        f.write_str(&self.owner)?;
        f.write_str("/")?;
        f.write_str(&self.repo)?;
        if let Some(tag) = &self.tag {
            f.write_str("/")?;
            f.write_str(tag)?;
        };
        Ok(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Url {
        fn new(provider: Provider, owner: &str, repo: &str, tag: Option<&str>) -> Self {
            Self {
                provider,
                owner: owner.into(),
                repo: repo.into(),
                tag: tag.map(|v| v.to_string()),
            }
        }
    }

    fn assert_url(input: &str, output: Url) {
        let val = Url::try_from(String::from(input));
        assert_eq!(val.unwrap(), output, "input: '{input}'");
    }

    #[test]
    fn github() {
        assert_url(
            "https://github.com/juliamertz/nixpins",
            Url::new(Provider::Github, "juliamertz", "nixpins", None),
        );
        assert_url(
            "github:juliamertz/nixpins",
            Url::new(Provider::Github, "juliamertz", "nixpins", None),
        );
        assert_url(
            "https://github.com/juliamertz/nixpins/e8410439655b74b97038352a8d3ec2d4c8a17fe3",
            Url::new(
                Provider::Github,
                "juliamertz",
                "nixpins",
                Some("e8410439655b74b97038352a8d3ec2d4c8a17fe3"),
            ),
        );
        assert_url(
            "github:juliamertz/nixpins/e8410439655b74b97038352a8d3ec2d4c8a17fe3",
            Url::new(
                Provider::Github,
                "juliamertz",
                "nixpins",
                Some("e8410439655b74b97038352a8d3ec2d4c8a17fe3"),
            ),
        );
    }
}
