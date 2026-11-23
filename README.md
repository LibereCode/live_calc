# live calc

> Calc is short for "calculator", btw.

... and now it's live!

Made with rusty rats.

## setup

```sh shell
# (interactive shell)
# Working directory is this directory:
devenv allow
```

```vim cmdline
" (cmd-line)
" edition first file with `neovim`
:trust
```

## Features

1. Base operators (+, -, \*, /)
1. Exponent (^)
1. Respect [PEDMAS](https://en.wikipedia.org/wiki/Order_of_operations)
   - For implemented features, have yet to implement other PEDMAS related features.
     See below.
1. Can input decimals.

## TODO

### Implement

- [x] _decimals_ (dots)
  - [ ] Better _floating numbers_ (rounding? More precise?)
- [x] _exponents_
- [ ] _brackets_
- [ ] _variables_
- [ ] Scroll _history_

### FIXME

- [ ] The _better-normal-mode-delete-char_ I did and have saved somewhere...
- [ ] Better _UI-layout_
- [ ] Bug where \*|/|^ with negative numbers doesn't work.

## LICENSE

Copyleft (🄯) 2026 LibereCode. All Rights Reserved.\
Licensed under the **EUPL-1.2**. See [the LICENSE](./LICENSE) for details.
