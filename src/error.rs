use thiserror::Error;
use reqwest;
use std::io;
use git2;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    #[error("Api request failed")]
    ApiError(String),
    #[error("Git Error")]
    GitIoError(#[from] io::Error),
    #[error("Git command failed")]
    GitError,
    #[error("Git clone failed")]
    GitCloneError(#[from] git2::Error),
}
