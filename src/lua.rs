mod language_servers;

use language_servers::{EmmyLuaLs, LuaLs};
use zed_extension_api::{
    self as zed, lsp::CompletionKind, settings::LspSettings, CodeLabel, CodeLabelSpan,
    LanguageServerId, Result,
};

struct LuaExtension {
    lua_ls: LuaLs,
    emmylua_ls: EmmyLuaLs,
}

impl zed::Extension for LuaExtension {
    fn new() -> Self {
        Self {
            lua_ls: LuaLs::new(),
            emmylua_ls: EmmyLuaLs::new(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            LuaLs::SERVER_ID => self
                .lua_ls
                .language_server_command(language_server_id, worktree),
            EmmyLuaLs::SERVER_ID => self
                .emmylua_ls
                .language_server_command(language_server_id, worktree),
            server_id => Err(format!("unknown language server: {server_id}")),
        }
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        let label_len = completion.label.len();
        let (label_detail, label_description) = completion
            .label_details
            .map(|details| (details.detail, details.description))
            .unwrap_or_default();
        let detail = completion.detail.filter(|detail| {
            detail != &completion.label
                && label_detail.as_ref() != Some(detail)
                && label_description.as_ref() != Some(detail)
        });
        let (code, mut spans, filter_len) = match completion.kind? {
            CompletionKind::Method | CompletionKind::Function => {
                let name_len = completion.label.find('(').unwrap_or(completion.label.len());
                (
                    completion.label,
                    vec![CodeLabelSpan::code_range(0..label_len)],
                    name_len,
                )
            }
            CompletionKind::Field | CompletionKind::Property => (
                Default::default(),
                vec![CodeLabelSpan::literal(
                    completion.label,
                    Some("property".into()),
                )],
                label_len,
            ),
            _ => return None,
        };

        if let Some(label_detail) = label_detail {
            spans.push(CodeLabelSpan::literal(label_detail, None));
        }

        if let Some(detail) = detail {
            spans.push(CodeLabelSpan::literal(" ", None));
            spans.push(CodeLabelSpan::literal(detail, None));
        }

        if let Some(label_description) = label_description {
            spans.push(CodeLabelSpan::literal(" ", None));
            spans.push(CodeLabelSpan::literal(label_description, None));
        }

        Some(CodeLabel {
            spans,
            filter_range: (0..filter_len).into(),
            code,
        })
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<CodeLabel> {
        // TODO: Include server-provided symbol detail (such as function parameters
        // from LuaLS and EmmyLuaLS) when zed_extension_api exposes it on `lsp::Symbol`.
        let highlight = match symbol.kind {
            zed::lsp::SymbolKind::Method => Some("function.method"),
            zed::lsp::SymbolKind::Function => Some("function"),
            zed::lsp::SymbolKind::Property
            | zed::lsp::SymbolKind::Field
            | zed::lsp::SymbolKind::Key
            | zed::lsp::SymbolKind::EnumMember => Some("property"),
            zed::lsp::SymbolKind::Class
            | zed::lsp::SymbolKind::Interface
            | zed::lsp::SymbolKind::Enum
            | zed::lsp::SymbolKind::Struct
            | zed::lsp::SymbolKind::TypeParameter
            | zed::lsp::SymbolKind::Module
            | zed::lsp::SymbolKind::Namespace
            | zed::lsp::SymbolKind::Package => Some("type"),
            zed::lsp::SymbolKind::Constructor => Some("constructor"),
            zed::lsp::SymbolKind::Constant => Some("constant"),
            zed::lsp::SymbolKind::Variable
            | zed::lsp::SymbolKind::String
            | zed::lsp::SymbolKind::Number
            | zed::lsp::SymbolKind::Boolean
            | zed::lsp::SymbolKind::Array
            | zed::lsp::SymbolKind::Object
            | zed::lsp::SymbolKind::Null => Some("variable"),
            zed::lsp::SymbolKind::Operator => Some("operator"),
            _ => None,
        };
        let name_len = symbol.name.len();
        Some(CodeLabel {
            spans: vec![CodeLabelSpan::literal(
                symbol.name,
                highlight.map(str::to_string),
            )],
            filter_range: (0..name_len).into(),
            code: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        LspSettings::for_worktree(server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.initialization_options.clone())
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        LspSettings::for_worktree(server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings.clone())
    }
}

zed::register_extension!(LuaExtension);
