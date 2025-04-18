use zed::settings::LspSettings;
use zed_extension_api::serde_json::Value;
use zed_extension_api::{self as zed, Result};

struct Lean4Extension;

impl zed::Extension for Lean4Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Check and return the binary path specified in LSP settings
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path {
                return Ok(zed::Command {
                    command: path,
                    args: binary_settings.arguments.unwrap_or_else(|| vec![]),
                    env: worktree.shell_env(),
                });
            }
        }

        // Check and return the binary path in PATH environment
        let path = worktree
            .which("lake")
            .ok_or_else(|| "Lean must be installed via Elan".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["serve".into()],
            env: worktree.shell_env(),
        })
        
        // TODO: Install, uninstall and update Elan like VSCode
        // It seems Zed extension API doesn't support executing commands like 'elan-init -y --default-toolchain nightly'
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<Value>> {
        LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<Value>> {
        LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings)
    }
}

zed::register_extension!(Lean4Extension);
