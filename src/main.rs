pub mod gitea;
pub mod config;
pub mod error;

use std::env;
use std::process::Command;
use tempdir::TempDir;
use git2::{FetchOptions, RemoteCallbacks, Cred, build::RepoBuilder};
use config::Config;
use error::Error;
use gitea::Gitea;

/// Create repository bundle
/// # Arguments
/// * `url` - git clone url
/// * `path` - path to bundle file
/// * `token` - gitea access token
fn bundle_repo(url: &str, path: &str, token: &str) -> Result<(), Error> {
    let tmp_dir = TempDir::new("gitea-bundle").unwrap();

    let mut cb = RemoteCallbacks::new();
    cb.credentials(|_, _, _| Cred::userpass_plaintext("git", token));
    let mut fo = FetchOptions::default();
    fo.remote_callbacks(cb);

    RepoBuilder::new()
        .fetch_options(fo)
        .bare(true)
        .remote_create(|repo,name,url| repo.remote_with_fetch(name, url, "+refs/*:refs/*"))
        .clone(url, tmp_dir.path())?;
    
    let output = Command::new("git")
        .args(["bundle", "create", path, "--all"])
        .current_dir(tmp_dir.path())
        .output()?
        ;
    if !output.status.success() {
        Err(Error::GitError)
    } else {
        Ok(())
    }
}

/// Convert clone url to bundle file name
/// # Arguments
/// * `url` - git clone url
fn url_to_path(url: &str) -> String {
    let v: Vec<&str> = url.split('/').collect();
    let repo_name = v[v.len()-1].replace(".git", ".bundle");
    let repo_user = v[v.len()-2];
    format!("{repo_user}_{repo_name}")
}

fn run(config: Config, gitea: Gitea) -> Result<(), Error> {
    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();

    for org in gitea.get_orgs()? {
        for r in gitea.get_org_repos(&org)? {
            println!("Bundling {r}");
            let p = format!("{}/{}", cwd, url_to_path(&r));
            bundle_repo(&r, &p, &config.token)?;
        }
    }

    for user in gitea.get_users()? {
        for r in gitea.get_user_repos(&user)? {
            println!("Bundling {r}");
            let p = format!("{}/{}", cwd, url_to_path(&r));
            bundle_repo(&r, &p, &config.token)?;
        }
    }
    Ok(())
}

fn main() {
    let config = Config::from_args();
    let gitea = Gitea::build(config.host.clone(), config.token.clone());

    if let Err(e) = run(config, gitea) {
        println!("Bundling failed: {e}");
    }
}
