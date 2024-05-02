use self::{contours::Contours, kmeans::KMeans, threshold::Threshold};
use anyhow::Result;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

/// Config
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub kmeans: KMeans,
    pub threshold: Threshold,
    pub contours: Contours,
}

impl Config {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Ok(from_reader(file)?)
    }
}

mod kmeans {
    use serde::{Deserialize, Serialize};

    /// K-means
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct KMeans {
        pub iterations: usize,
        pub k: usize,
        pub runs: u64,
        pub seed: u64,
    }

    impl Default for KMeans {
        fn default() -> Self {
            Self {
                iterations: 20,
                k: 5,
                runs: 1,
                seed: 0,
            }
        }
    }
}

mod contours {
    use opencv::imgproc::{CHAIN_APPROX_SIMPLE, RETR_EXTERNAL, RETR_TREE};
    use serde::{Deserialize, Serialize};

    /// Contours
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct Contours {
        pub mode: i32,
        pub method: i32,
        pub min_area: f64,
    }

    impl Default for Contours {
        fn default() -> Self {
            Self {
                mode: RETR_EXTERNAL,
                method: CHAIN_APPROX_SIMPLE,
                min_area: 0.0,
            }
        }
    }

    #[repr(i32)]
    pub enum Mode {
        RetrTree = RETR_TREE,
        RetrExternal = RETR_EXTERNAL,
    }
}

mod threshold {
    use opencv::imgproc::{
        ADAPTIVE_THRESH_GAUSSIAN_C, ADAPTIVE_THRESH_MEAN_C, THRESH_BINARY_INV, THRESH_OTSU,
    };
    use serde::{Deserialize, Serialize};

    /// Threshold
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct Threshold {
        pub thresh: f64,
        pub max: f64,
        pub r#type: i32,
    }

    impl Default for Threshold {
        fn default() -> Self {
            Self {
                thresh: 0.0,
                max: 255.0,
                r#type: THRESH_BINARY_INV | THRESH_OTSU,
            }
        }
    }

    #[repr(i32)]
    pub enum Type {
        Gaussian = ADAPTIVE_THRESH_GAUSSIAN_C,
        Mean = ADAPTIVE_THRESH_MEAN_C,
    }
}
