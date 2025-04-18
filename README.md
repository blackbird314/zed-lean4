# Zed Lean 4

A [Lean(4) Theorem Prover](https://lean-lang.org/) extension for [Zed](https://zed.dev).

- Tree-sitter: [tree-sitter-lean](https://github.com/Julian/tree-sitter-lean)
- Language Server: [Lean Language Server](https://github.com/leanprover/lean4/tree/master/src/Lean/Server)

## Install Lean Toolchain

The Lean Language Server is integrated with Lean Toolchain. It's recommended to install nightly version via [elan](https://github.com/leanprover/elan?tab=readme-ov-file#installation):

```sh
elan default nightly
lean --version
```

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs. 

Note the tree-sitter-lean is experimental and needs improvement.

