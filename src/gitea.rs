use std::vec;
use reqwest::blocking::Response;
use serde::Deserialize;
use reqwest::header::AUTHORIZATION;

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

#[derive(Deserialize, Debug)]
struct Repo {
    clone_url: String
}

impl Gitea {
    pub fn build(host: String, token: String) -> Self {
        Self { host, token }
    }

    pub fn api_get(&self, url: String) -> Response {
        let client = reqwest::blocking::Client::new();
        client
            .get(format!("{}{}", self.host, url))
            .header(AUTHORIZATION, format!("token {}", self.token))
            .send()
            .unwrap()
    }

    pub fn get_orgs(&self) -> Vec<String> {
        let orgs: Vec<Org> = self.api_get(String::from("/api/v1/orgs")).json().unwrap();
        let mut ret: Vec<String> = vec![];
        for org in orgs {
            ret.push(org.username)
        }
        ret
    }

    pub fn get_users(&self) -> Vec<String> {
        let users: Vec<User> = self.api_get(String::from("/api/v1/admin/users")).json().unwrap();
        let mut ret: Vec<String> = vec![];
        for user in users {
            ret.push(user.login)
        }
        ret
    }

    pub fn get_org_repos(&self, org: String) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        for page in 1.. {
            let repos: Vec<Repo> = self.api_get(format!("/api/v1/orgs/{org}/repos?page={page}")).json().unwrap();
            if repos.is_empty() {
                break;
            }
            for repo in repos {
                ret.push(repo.clone_url);
            }
        }
        ret
    }

    pub fn get_user_repos(&self, user: String) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        for page in 1.. {
            let repos: Vec<Repo> = self.api_get(format!("/api/v1/users/{user}/repos?page={page}")).json().unwrap();
            if repos.is_empty() {
                break;
            }
            for repo in repos {
                ret.push(repo.clone_url);
            }
        }
        ret
    }
}
