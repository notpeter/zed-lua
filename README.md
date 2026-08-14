# Zed Lua

A [Lua](https://www.lua.org/) extension for [Zed](https://zed.dev).

## Language servers

The extension supports multiple language servers.

| Name      | GitHub                                                                                | Settings ID           | Binary                |
| --------- | ------------------------------------------------------------------------------------- | --------------------- | --------------------- |
| LuaLS     | [LuaLS/lua-language-server](https://github.com/LuaLS/lua-language-server)             | `lua-language-server` | `lua-language-server` |
| EmmyLuaLS | [EmmyLuaLs/emmylua-analyzer-rust](https://github.com/EmmyLuaLs/emmylua-analyzer-rust) | `emmylua_ls`          | `emmylua_ls`          |

If Zed finds the binary for an LSP in your PATH (e.g. `which emmylua_ls` or `which lua-language-server`) it will use that version, if not it will automatically download managed copies of the newest version via GitHub releases.

### Language server selection

You should only enable one language server at a time. For backwards compatability, Zed defaults to `LuaLS`.

If you would like to use EmmyLuaLS instead, you can specify your preferred language server via Zed Settings:

```json
{
  "languages": {
    "Lua": {
      "language_servers": ["emmylua_ls", "!lua-language-server", "..."]
    }
  }
}
```

Explicitly use LuaLS:

```json
{
  "languages": {
    "Lua": {
      "language_servers": ["lua-language-server", "!emmylua_ls", "..."]
    }
  }
}
```

Or to use no language server:

```json
{
  "languages": {
    "Lua": {
      "enable_language_server": false
    }
  }
}
```


### Advanced CLI Settings

You can also specify optional `binary` configuration per language-server if required:

```json
{
  "lsp": {
    "emmylua_ls": {
      "binary": {
        "path": "/home/somebody/.cargo/bin/emmylua_ls",
        "arguments": ["--log-level", "debug"],
        "env": {
          "RUST_LOG": "debug"
        }
      }
    }
  }
}
```

## Links

- Official [Zed Lua Language Docs](https://zed.dev/docs/languages/lua)
- [`.luarc.json` Documentation](https://luals.github.io/wiki/configuration/#luarcjson-file) and [`.luarc.json` Schema](https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json)
- [`.emmyrc.json` Docucmentation](https://github.com/EmmyLuaLs/emmylua-analyzer-rust/blob/main/docs/config/emmyrc_json_EN.md) and [`.emmyrc.json` Schema](https://raw.githubusercontent.com/EmmyLuaLs/emmylua-analyzer-rust/refs/heads/main/crates/emmylua_code_analysis/resources/schema.json)
- Luau Extension: [4teapo/zed-luau ](https://github.com/4teapo/zed-luau)
- Fennel Extension: [notpeter/fennel](https://github.com/notpeter/fennel-zed/) 

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
