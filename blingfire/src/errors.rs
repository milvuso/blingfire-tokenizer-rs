
use snafu::Snafu;
use std::result::Result as StdResult;

/// Error enum encoding tokenization errors.
#[derive(Debug, Snafu, PartialEq)]
#[snafu(visibility = "pub")]