use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Read
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Read {
    pub path: PathBuf,
}
