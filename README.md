# Zed Lean 4

A [Lean(4) Theorem Prover](https://lean-lang.org/) extension for [Zed](https://zed.dev).

- Tree-sitter: [tree-sitter-lean](https://github.com/Julian/tree-sitter-lean)
- Language Server: [Lean Language Server](https://github.com/leanprover/lean4/tree/master/src/Lean/Server)

## Lean Toolchain

The Lean Language Server is integrated with the Lean toolchain. It is recommended to manage Lean toolchain versions via [_elan_](https://github.com/leanprover/elan?tab=readme-ov-file#installation). For example, you can install the nightly Lean version by running:

```sh
elan default nightly
lean --version
```

<!--**If elan is not detected, this extension will automatically install it and set the default Lean version to the latest stable release.** During installation, elan automatically adds itself to your PATH environment variable.

The Lean toolchain can be completely removed, deleting all installed files and reverting any environment variable changes made during installation. To do so, simply run:

```sh
elan self uninstall
```-->

## Features

### Highlight

For optimal highlighting, add `"semantic_tokens": "combined"` in `settings.json`, which will enable LSP semantic tokens highlight.

### Abbreviation

Abbreviation (unicode character) insertion is supported by [snippets](https://github.com/owlx56/zed-lean4/blob/main/snippets/lean%204.json).

## TODO list

- **_Tree-sitter-lean is experimental and needs improvement_**
- Implement infoview like [VSCode](https://github.com/leanprover/vscode-lean4) and [Neovim](https://github.com/Julian/lean.nvim)
- Update and uninstall _elan_

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
