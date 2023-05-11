use thiserror::Error;
use reqwest;
use std::io;
use git2;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Generic error")]
    Generic,
    #[error("Request error: {0}")]
    RequestError(reqwest::Error),
    #[error("Api request failed: {0}")]
    ApiError(String),
    #[error("Git Error: {0}")]
    GitIoError(io::Error),
    #[error("Git command failed")]
    GitError,
    #[error("Git clone failed: {0}")]
    GitCloneError(git2::Error),
}
