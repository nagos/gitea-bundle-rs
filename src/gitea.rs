use std::iter;
pub struct Gitea {
    token: String,
}

impl Gitea {
    pub fn build(token: String) -> Self {
        Self { token }
    }

    pub fn into_repos(self) -> GiteaReposIter {
        todo!()
    }
}

pub struct GiteaReposIter;

impl iter::Iterator for GiteaReposIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
