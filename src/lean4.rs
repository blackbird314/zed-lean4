use std::{env, fs, path::PathBuf};
use zed_extension_api::{self as zed, Result, serde_json::Value, settings::LspSettings};

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
        let shell_env = worktree.shell_env();

        // Check language server path specified in LSP settings
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        if let Some(binary_settings) = lsp_settings.binary
            && let Some(path) = binary_settings.path
        {
            return Ok(zed::Command {
                command: path,
                args: binary_settings.arguments.unwrap_or_default(),
                env: vec![],
            });
        }

        // Check if lake is available in PATH
        if let Some(path) = worktree.which("lake") {
            return Ok(zed::Command {
                command: path,
                args: vec!["serve".into(), "--".into()],
                env: vec![],
            });
        }

        // Check ELAN_HOME or default path
        let elan_home = shell_env
            .iter()
            .find_map(|(k, v)| (k == "ELAN_HOME").then_some(PathBuf::from(v)))
            .or_else(|| {
                let home = shell_env
                    .iter()
                    .find_map(|(k, v)| (k == "HOME" || k == "USERPROFILE").then_some(v))?;
                Some(PathBuf::from(home).join(".elan"))
            })
            .ok_or("Failed to find ELAN_HOME, HOME, or USERPROFILE")?;

        let (platform, arch) = zed::current_platform();

        let lake_path = elan_home.join("bin").join(match platform {
            zed::Os::Windows => "lake.exe",
            zed::Os::Mac | zed::Os::Linux => "lake",
        });
        let lake_path_str = lake_path.to_string_lossy().to_string();

        if worktree.which(&lake_path_str).is_some() {
            return Ok(zed::Command {
                command: lake_path_str,
                args: vec!["serve".into(), "--".into()],
                env: vec![],
            });
        }

        // Download and Install Lean 4 toolchain
        let release = zed::latest_github_release(
            "leanprover/elan",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name: String = format!(
            "elan-{}-{}.{}",
            match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X8664 => "x86_64",
                zed::Architecture::X86 =>
                    return Err("32-bit x86 architecture is not supported by elan".into()),
            },
            match platform {
                zed::Os::Windows => "pc-windows-msvc",
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
            },
            match platform {
                zed::Os::Windows => "zip",
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("No asset found matching: {:?}", asset_name))?;

        let version_dir = format!("elan-{}", release.version);
        let elan_init_name = match platform {
            zed::Os::Windows => "elan-init.exe",
            zed::Os::Mac | zed::Os::Linux => "elan-init",
        };

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );
        zed::download_file(
            &asset.download_url,
            &version_dir,
            match platform {
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
            },
        )
        .map_err(|e| format!("Failed to download file: {e}"))?;

        let cwd =
            env::current_dir().map_err(|e| format!("Failed to get current directory: {e}"))?;
        let elan_init_path = cwd.join(&version_dir).join(elan_init_name);
        let elan_init_path_str = elan_init_path.to_string_lossy().to_string();

        zed::make_file_executable(&elan_init_path_str)?;
        zed::Command::new(&elan_init_path_str)
            .args(["-y", "--default-toolchain", "leanprover/lean4:stable"])
            .output()?;

        fs::remove_dir_all(&version_dir).ok();

        Ok(zed::Command {
            command: lake_path_str,
            args: vec!["serve".into(), "--".into()],
            env: vec![],
        })
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
