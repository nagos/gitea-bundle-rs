pub mod gitea;
pub mod config;
pub mod error;

use gitea::Gitea;
use std::env;
use tempdir::TempDir;
use git2::{FetchOptions, RemoteCallbacks};
use git2::Cred;
use git2::build::RepoBuilder;
use std::process::Command;
use config::Config;
use error::Error;

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
        .clone(url, tmp_dir.path())
        .map_err(Error::GitCloneError)?;
    
    let output = Command::new("git")
        .args(["bundle", "create", path, "--all"])
        .current_dir(tmp_dir.path())
        .output()
        .map_err(Error::GitIoError)?
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
    let repo_name = v[v.len()-1];
    let repo_user = v[v.len()-2];
    format!("{repo_user}_{repo_name}")
}

fn run() -> Result<(), Error> {
    let config = Config::from_args();
    
    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let gitea = Gitea::build(config.host.clone(), config.token.clone());

    let orgs = gitea.get_orgs()?;
    for org in orgs {
        let repos = gitea.get_org_repos(org.clone())?;
        for r in repos {
            println!("Bundling {r}");
            let p = format!("{}/{}", cwd, url_to_path(&r));
            bundle_repo(&r, &p, &config.token)?;
        }
    }

    let users = gitea.get_users()?;
    for user in users {
        let repos = gitea.get_user_repos(user.clone())?;
        for r in repos {
            println!("Bundling {r}");
            let p = format!("{}/{}", cwd, url_to_path(&r));
            bundle_repo(&r, &p, &config.token)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Bundling failed: {e}");
    }
}
