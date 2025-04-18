# Zed Lean 4

A [Lean(4) Theorem Prover](https://lean-lang.org/) extension for [Zed](https://zed.dev).

- Tree-sitter: [tree-sitter-lean](https://github.com/Julian/tree-sitter-lean)
- Language Server: [Lean Language Server](https://github.com/leanprover/lean4/tree/master/src/Lean/Server)

## Install Lean Toolchain

The Lean Language Server is integrated with Lean Toolchain. It's recommended to install nightly version via [_elan_](https://github.com/leanprover/elan?tab=readme-ov-file#installation):

```sh
elan default nightly
lean --version
```

## TODO list

- **_Tree-sitter-lean is experimental and needs improvement._**
- Install, update and ninstall _elan_ like VSCode
- Implement infoview like [VSCode](https://github.com/leanprover/vscode-lean4) and [Neovim](https://github.com/Julian/lean.nvim).

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.