// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use anyhow::{Result, anyhow};
use scraper::{Html, Selector};
use std::str::FromStr;
use url::Url;
use version::Version;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub struct UnicodeVersion {
    pub url: Url,
    pub version: Version,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl UnicodeVersion {
    const URL_BASE: &str = "https://www.unicode.org/Public/";

    pub fn from_str(s: &str) -> Result<Self> {
        let Ok(version) = Version::from_str(s) else {
            return Err(anyhow!(
                "failed to parse the provided string as a version: {s}"
            ));
        };
        Ok(Self::new(version))
    }

    pub fn latest() -> Result<Self> {
        let versions = Self::list()?;
        versions
            .into_iter()
            .last()
            .ok_or_else(|| anyhow!("could not find any version of the Unicode standard"))
    }

    pub fn list() -> Result<Vec<Self>> {
        let mut versions = Vec::new();
        let html = reqwest::blocking::get(Self::url_base())?.text()?;
        let document = Html::parse_document(&html);
        let Ok(selector) = Selector::parse("a[href]") else {
            return Err(anyhow!("failed to select href elements from returned HTML"));
        };
        for element in document.select(&selector) {
            let label = element.text().collect::<String>().trim().to_owned();
            let label = label.trim_end_matches('/');
            let Ok(version) = Version::from_str(label) else {
                continue;
            };
            versions.push(Self::new(version))
        }
        versions.sort_by_key(|v| (v.version.major, v.version.minor, v.version.patch));
        Ok(versions)
    }

    pub fn new(version: Version) -> Self {
        let verstr = format!("{version}/");
        let url = Self::url_base().join(&verstr).expect(
            "implementation error: the version string was unable to join with the URL base",
        );
        Self { url, version }
    }

    fn url_base() -> Url {
        Url::parse(Self::URL_BASE).expect("implementation error: the `URL_BASE` is not a valid URL")
    }
}
