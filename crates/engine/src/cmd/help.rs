//! Help → project links (this app's GitHub repository). The command returns the URL; the frontend
//! opens it.

use serde_json::{Value, json};

use super::*;

/// The app's repository name on GitHub.
pub const APP_ID: &str = "vectorcraft";

/// This app's source repository.
pub fn github_url() -> String {
    format!("https://github.com/VoltrunGames/{APP_ID}")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.github", "Tend's Vector on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {github}", always, |_, _| {
            Ok(json!({ "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/VoltrunGames/vectorcraft");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["github"], "https://github.com/VoltrunGames/vectorcraft");
    }
}
