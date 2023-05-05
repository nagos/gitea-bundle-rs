pub mod gitea;

use gitea::Gitea;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Org {
    username: String
}

fn main() {
    let token = env::var("GITEA_TOKEN").unwrap();
    let request_url = format!("http://dev.profitt.ru/api/v1/orgs?access_token={token}");
    
    println!("{}", request_url);
    let response = reqwest::blocking::get(&request_url).unwrap();


    let users: Vec<Org> = response.json().unwrap();
    println!("{:?}", users);

    let gitea = Gitea::build(token);
    
    for repo in gitea.into_repos() {
        dbg!(repo);
    }
}
