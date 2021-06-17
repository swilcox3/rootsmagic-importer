use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("{0}")]
    HTTPError(#[from] reqwest::Error),
    #[error("The HTTP request {0} returned {1}")]
    HTTPStatusError(String, reqwest::StatusCode),
    #[error("There are no matches to display")]
    NoMatchesError,
    #[error("{0}")]
    URLEncodeError(#[from] serde_urlencoded::ser::Error),
    #[error("{0}")]
    GuiError(#[from] fltk::prelude::FltkError),
    #[error("{0}")]
    JsonError(#[from] serde_json::Error)
}
