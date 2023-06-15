pub mod config;

use std::env;
use anyhow::{Context, Result};
use config::Config;
use gitea_bundle::gitea::Gitea;
use gitea_bundle::{url_to_path, bundle_repo};

fn bundle_orgs(config: &Config, gitea: &Gitea, cwd: &str) -> Result<()> {
    for org in gitea.get_orgs()? {
        for r in gitea.get_org_repos(&org)? {
            println!("Bundling {r}");
            let p = format!("{}/{}", cwd, url_to_path(&r));
            bundle_repo(&r, &p, &config.token)?;
        }
    }

    Ok(())
}

fn bundle_users(config: &Config, gitea: &Gitea, cwd: &str) -> Result<()> {
    for user in gitea.get_users()? {
        for r in gitea.get_user_repos(&user)? {
            println!("Bundling {r}");
            let p = format!("{}/{}", cwd, url_to_path(&r));
            bundle_repo(&r, &p, &config.token)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let config = Config::from_args();
    let gitea = Gitea::build(&config.host, &config.token);
    let current_dir = env::current_dir()?;
    let cwd = current_dir.to_str().unwrap();

    bundle_orgs(&config, &gitea, cwd).context("Bundling orgs failed")?;
    bundle_users(&config, &gitea, cwd).context("Bundling users failed")?;

    Ok(())
}
