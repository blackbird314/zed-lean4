# Zed Lean 4

A [Lean(4) Theorem Prover](https://lean-lang.org/) extension for [Zed](https://zed.dev).

- Tree-sitter: [tree-sitter-lean](https://github.com/Julian/tree-sitter-lean)
- Language Server: [Lean Language Server](https://github.com/leanprover/lean4/tree/master/src/Lean/Server)

## Lean Toolchain

The Lean Language Server is integrated with the Lean toolchain. It is recommended to manage Lean toolchain versions via [_elan_](https://github.com/leanprover/elan?tab=readme-ov-file#installation). For example, you can install the stable Lean version by running:

```sh
elan default stable
lean --version
```

## Features

### Highlight

For optimal highlighting, set `"semantic_tokens"` to `"combined"`, which will enable LSP semantic tokens highlight.

### Abbreviation

Abbreviation (unicode character) insertion is supported by [snippets](https://github.com/owlx56/zed-lean4/blob/main/snippets/lean%204.json).

### Installation

Firstly, this extension will check `path` and `arguments` in `lsp.lean4-lsp.binary` settings. Then it will search `lake` in $PATH, try $ELAN_HOME and default directory.

If Lean 4 is not detected, this extension can automatically install elan with default toolchain. To enable this feature, set `"elan_auto_install"` to `true`. You can also specify the default toolchain by setting `"elan_default_toolchain"` to `"stable"`, `"nightly"` or `"v4.28.0"`.

During installation, the extension will automatically add `~/.elan` to your $PATH. The Lean toolchain can be completely removed by running `elan self uninstall`.

## Settings Example

```json
// Zed settings
{
  "semantic_tokens": "combined",
  "lsp": {
    "lean4-lsp": {
      "binary": {
        "path": "/path/to/your/.elan/bin/lake",
        "arguments": ["serve", "--"]
      },
      "settings": {
        "elan_auto_install": true,
        "elan_default_toolchain": "stable"
      }
    }
  }
}
```

## TODO list

- **_Tree-sitter-lean is experimental and needs improvement_**
- Implement infoview like [VSCode](https://github.com/leanprover/vscode-lean4) and [Neovim](https://github.com/Julian/lean.nvim)
- Update and uninstall _elan_

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
