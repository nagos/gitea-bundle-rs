pub mod gitea;

use gitea::Gitea;
use std::env;


fn main() {
    let token = env::var("GITEA_TOKEN").unwrap();
    let url = env::var("GITEA_HOST").unwrap();

    let gitea = Gitea::build(url, token);
    let orgs = gitea.get_orgs();
    for org in orgs {
        let repos = gitea.get_org_repos(org.clone());
        for r in repos {
            println!("{org} {r}");
        }
    }
    let users = gitea.get_users();
    for user in users {
        let repos = gitea.get_user_repos(user.clone());
        for r in repos {
            println!("{user} {r}");
        }
    }
}
