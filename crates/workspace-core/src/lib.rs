use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceRuntimeCapabilities {
    pub product_id: &'static str,
    pub can_open_projects: bool,
    pub can_read_workspace_files: bool,
    pub can_run_processes: bool,
    pub can_use_git: bool,
}

pub const CODE_RUNTIME: WorkspaceRuntimeCapabilities = WorkspaceRuntimeCapabilities {
    product_id: "tench-code",
    can_open_projects: true,
    can_read_workspace_files: true,
    can_run_processes: true,
    can_use_git: true,
};

pub const ONE_RUNTIME: WorkspaceRuntimeCapabilities = WorkspaceRuntimeCapabilities {
    product_id: "tench-one",
    can_open_projects: false,
    can_read_workspace_files: false,
    can_run_processes: true,
    can_use_git: false,
};

pub const STORY_RUNTIME: WorkspaceRuntimeCapabilities = WorkspaceRuntimeCapabilities {
    product_id: "tench-story",
    can_open_projects: false,
    can_read_workspace_files: true,
    can_run_processes: false,
    can_use_git: false,
};

pub fn workspace_runtime_products() -> [WorkspaceRuntimeCapabilities; 3] {
    [CODE_RUNTIME, ONE_RUNTIME, STORY_RUNTIME]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_runtime_owns_process_and_git_capabilities() {
        const { assert!(CODE_RUNTIME.can_run_processes) };
        const { assert!(CODE_RUNTIME.can_use_git) };
        const { assert!(!STORY_RUNTIME.can_use_git) };
    }
}
