use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackConfig {
    pub source_dir: PathBuf,
    pub output_path: PathBuf,
    pub package_name: String,
    pub version: String,
    pub compression_level: u32,
    pub threads: Option<usize>,
    pub exclude_patterns: Vec<String>,
    pub install_dir: String,
    // Qt IFW 风格打包选项
    #[serde(default)]
    pub use_qt_ifw_style: bool,
    #[serde(default)]
    pub install_root: Option<String>,
    #[serde(default)]
    pub qmake_args: Vec<String>,
    #[serde(default)]
    pub make_args: Vec<String>,
    #[serde(default)]
    pub bindist_script: Option<String>,
}

impl Default for PackConfig {
    fn default() -> Self {
        Self {
            source_dir: PathBuf::from("./"),
            output_path: PathBuf::from("./output.run"),
            package_name: String::from("package"),
            version: String::from("1.0.0"),
            compression_level: 3,
            threads: None,
            exclude_patterns: Vec::new(),
            install_dir: String::from("/opt/package"),
            use_qt_ifw_style: false,
            install_root: None,
            qmake_args: Vec::new(),
            make_args: Vec::new(),
            bindist_script: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackProgress {
    pub current_file: String,
    pub files_processed: usize,
    pub total_files: usize,
    pub bytes_processed: u64,
    pub total_bytes: u64,
    pub compression_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackResult {
    pub success: bool,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub files_count: usize,
    pub duration_ms: u128,
}
