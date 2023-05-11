use std::vec;
use reqwest::blocking::Response;
use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use crate::error::Error;

/// Gitea api module
pub struct Gitea {
    token: String,
    host: String,
}

#[derive(Deserialize, Debug)]
struct Org {
    username: String
}

#[derive(Deserialize, Debug)]
struct User {
    login: String
}

#[derive(Deserialize, Debug, Default)]
struct ApiError {
    message: String
}

#[derive(Deserialize, Debug)]
struct Repo {
    clone_url: String
}

impl Gitea {
    /// Create struct
    /// * `host` - url of gitea host
    /// * `token` - gitea access token
    pub fn build(host: String, token: String) -> Self {
        Self { host, token }
    }

    /// Perform API request
    /// * `url` - API url, without host
    fn api_get(&self, url: String) -> Result<Response, Error> {
        let client = reqwest::blocking::Client::new();
        let result = client
            .get(format!("{}{}", self.host, url))
            .header(AUTHORIZATION, format!("token {}", self.token))
            .send()
            .map_err(Error::RequestError)?;
        
        if !result.status().is_success() {
            let msg: ApiError = result.json().unwrap_or_default();
            Err(Error::ApiError(msg.message))
        } else {
            Ok(result)
        }
    }

    /// List Gitea orgs
    pub fn get_orgs(&self) -> Result<Vec<String>, Error> {
        let orgs: Vec<Org> = self.api_get(String::from("/api/v1/orgs"))?.json().unwrap_or_default();
        let mut ret: Vec<String> = vec![];
        for org in orgs {
            ret.push(org.username)
        }
        Ok(ret)
    }

    /// List Gitea users
    pub fn get_users(&self) -> Result<Vec<String>, Error> {
        let users: Vec<User> = self.api_get(String::from("/api/v1/admin/users"))?.json().unwrap_or_default();
        let mut ret: Vec<String> = vec![];
        for user in users {
            ret.push(user.login)
        }
        Ok(ret)
    }

    /// Get org repositories
    /// * `org` - Organisation
    pub fn get_org_repos(&self, org: String) -> Result<Vec<String>, Error> {
        let mut ret: Vec<String> = vec![];
        for page in 1.. {
            let repos: Vec<Repo> = self.api_get(format!("/api/v1/orgs/{org}/repos?page={page}"))?.json().unwrap_or_default();
            if repos.is_empty() {
                break;
            }
            for repo in repos {
                ret.push(repo.clone_url);
            }
        }
        Ok(ret)
    }

    /// Get user repositories
    /// * `user` - User
    pub fn get_user_repos(&self, user: String) -> Result<Vec<String>, Error> {
        let mut ret: Vec<String> = vec![];
        for page in 1.. {
            let repos: Vec<Repo> = self.api_get(format!("/api/v1/users/{user}/repos?page={page}"))?.json().unwrap();
            if repos.is_empty() {
                break;
            }
            for repo in repos {
                ret.push(repo.clone_url);
            }
        }
        Ok(ret)
    }
}
