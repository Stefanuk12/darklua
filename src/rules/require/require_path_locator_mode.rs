use std::path::{Path, PathBuf};

use crate::nodes::FunctionCall;

pub trait RequirePathLocatorMode {
    fn get_source(&self, name: &str, rel: &Path) -> Option<PathBuf>;
    fn module_folder_name(&self) -> &str;
    fn match_path_require_call(&self, call: &FunctionCall, source: &Path) -> Option<PathBuf>;
}
