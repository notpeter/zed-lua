mod emmylua_ls;
mod lua_ls;

pub use emmylua_ls::EmmyLuaLs;
pub use lua_ls::LuaLs;

use std::fs;
use zed_extension_api::Result;

fn remove_old_server_versions(prefix: &str, current_version_dir: &str) -> Result<()> {
    let entries =
        fs::read_dir(".").map_err(|e| format!("failed to list working directory: {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("failed to load directory entry: {e}"))?;
        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            continue;
        };
        if is_old_server_version(file_name, prefix, current_version_dir) {
            fs::remove_dir_all(entry.path()).ok();
        }
    }
    Ok(())
}

fn is_old_server_version(file_name: &str, prefix: &str, current_version_dir: &str) -> bool {
    file_name.starts_with(prefix) && file_name != current_version_dir
}

#[cfg(test)]
mod tests {
    use super::is_old_server_version;

    #[test]
    fn old_version_cleanup_is_scoped_to_one_server() {
        assert!(is_old_server_version(
            "lua-language-server-3.0.0",
            "lua-language-server-",
            "lua-language-server-3.1.0"
        ));
        assert!(!is_old_server_version(
            "lua-language-server-3.1.0",
            "lua-language-server-",
            "lua-language-server-3.1.0"
        ));
        assert!(!is_old_server_version(
            "emmylua_ls-0.25.1",
            "lua-language-server-",
            "lua-language-server-3.1.0"
        ));
    }
}
