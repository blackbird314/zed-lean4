# Zed Lean 4

A [Lean(4) Theorem Prover](https://lean-lang.org/) extension for [Zed](https://zed.dev).

- Tree-sitter: [tree-sitter-lean](https://github.com/Julian/tree-sitter-lean)
- Language Server: [Lean Language Server](https://github.com/leanprover/lean4/tree/master/src/Lean/Server)

## Install Lean Toolchain

The Lean Language Server is integrated with the Lean toolchain. It is recommended to manage Lean versions via [_elan_](https://github.com/leanprover/elan?tab=readme-ov-file#installation). For example, you can install the nightly Lean version:

```sh
elan default nightly
lean --version
```

If elan is not detected, this extension will automatically install it and set the default Lean version to the latest stable release.

## Features

### Abbreviation

Abbreviation (unicode character) insertion is supported by snippets.

## TODO list

- **_Tree-sitter-lean is experimental and needs improvement._**
- Update and uninstall _elan_ like VSCode
- Implement infoview like [VSCode](https://github.com/leanprover/vscode-lean4) and [Neovim](https://github.com/Julian/lean.nvim).

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
