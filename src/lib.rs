pub mod gitea;
pub mod error;
use tempdir::TempDir;
use git2::{FetchOptions, RemoteCallbacks, Cred, build::RepoBuilder};
use std::process::Command;

use error::Error;

/// Convert clone url to bundle file name
/// # Arguments
/// * `url` - git clone url
pub fn url_to_path(url: &str) -> String {
    let v: Vec<&str> = url.split('/').collect();
    let repo_name = v[v.len()-1].replace(".git", ".bundle");
    let repo_user = v[v.len()-2];
    format!("{repo_user}_{repo_name}")
}

/// Create repository bundle
/// # Arguments
/// * `url` - git clone url
/// * `path` - path to bundle file
/// * `token` - gitea access token
pub fn bundle_repo(url: &str, path: &str, token: &str) -> Result<(), Error> {
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
