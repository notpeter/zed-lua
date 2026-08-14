use super::remove_old_server_versions;
use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

pub struct EmmyLuaLs {
    cached_binary_path: Option<String>,
}

impl EmmyLuaLs {
    pub const SERVER_ID: &str = "emmylua_ls";
    const BINARY_NAME: &str = "emmylua_ls";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?.binary;
        let args = binary
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_default();
        let env = binary
            .as_ref()
            .and_then(|binary| binary.env.clone())
            .map(|env| env.into_iter().collect())
            .unwrap_or_default();
        let configured_or_system_path = binary
            .and_then(|binary| binary.path)
            .or_else(|| worktree.which(Self::BINARY_NAME));
        let command = match configured_or_system_path {
            Some(path) => path,
            None => self.zed_managed_binary_path(language_server_id)?,
        };

        Ok(zed::Command { command, args, env })
    }

    fn zed_managed_binary_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "EmmyLuaLs/emmylua-analyzer-rust",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = Self::asset_name(platform, arch)?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("emmylua_ls-{}", release.version);
        let binary_path = format!(
            "{version_dir}/emmylua_ls{extension}",
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "",
                zed::Os::Windows => ".exe",
            },
        );

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            remove_old_server_versions("emmylua_ls-", &version_dir)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn asset_name(platform: zed::Os, arch: zed::Architecture) -> Result<String> {
        let platform_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "darwin-arm64",
            (zed::Os::Mac, zed::Architecture::X8664) => "darwin-x64",
            (zed::Os::Mac, zed::Architecture::X86) => {
                return Err("unsupported macOS architecture x86".into());
            }
            (zed::Os::Linux, zed::Architecture::Aarch64) => "linux-aarch64-glibc.2.17",
            (zed::Os::Linux, zed::Architecture::X8664) => "linux-x64-glibc.2.17",
            (zed::Os::Linux, zed::Architecture::X86) => {
                return Err("unsupported Linux architecture x86".into());
            }
            (zed::Os::Windows, zed::Architecture::Aarch64) => "win32-arm64",
            (zed::Os::Windows, zed::Architecture::X8664) => "win32-x64",
            (zed::Os::Windows, zed::Architecture::X86) => "win32-ia32",
        };
        let extension = match platform {
            zed::Os::Mac | zed::Os::Linux => "tar.gz",
            zed::Os::Windows => "zip",
        };
        Ok(format!("emmylua_ls-{platform_name}.{extension}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_its_own_server_id() {
        assert_eq!(EmmyLuaLs::SERVER_ID, "emmylua_ls");
    }

    #[test]
    fn maps_supported_release_assets() {
        assert_eq!(
            EmmyLuaLs::asset_name(zed::Os::Mac, zed::Architecture::Aarch64).unwrap(),
            "emmylua_ls-darwin-arm64.tar.gz"
        );
        assert_eq!(
            EmmyLuaLs::asset_name(zed::Os::Linux, zed::Architecture::X8664).unwrap(),
            "emmylua_ls-linux-x64-glibc.2.17.tar.gz"
        );
        assert_eq!(
            EmmyLuaLs::asset_name(zed::Os::Windows, zed::Architecture::X86).unwrap(),
            "emmylua_ls-win32-ia32.zip"
        );
    }
}
