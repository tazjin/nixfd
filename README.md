**Note**: Nix 2.0 contains a new `nix search` command which supports caching. You may want to use that instead of `nixfd`!

------------

nixfd
=====

Quick and dirty tool to cache and search through the Nix package/derivation database.

Every NixOS user is probably painfully aware that `nix-env -qaP` is incredibly slow, but it's also something that ends
up getting called quite often.

As a workaround, simply `cargo install` this repository and never worry again!

```sh
$ nixfd emacs
nixos.emacs                                                             emacs-25.3
nixos.emacs-all-the-icons-fonts                                         emacs-all-the-icons-fonts-3.1.1
nixos.emacs25-nox                                                       emacs-nox-25.3
nixos.python27Packages.ropemacs                                         python2.7-ropemacs-0.7
nixos.python36Packages.ropemacs                                         python3.6-ropemacs-0.7
``` 

The package cache is in `$XDG_CACHE_HOME/nixpkgs` (usually defaults to `~/.cache/nixpkgs`). The file is refreshed once
it is more than 12 hours old.

**Note**: Assumes you use [Nix](https://nixos.org/nix/) and have [ripgrep](https://github.com/BurntSushi/ripgrep) 
installed.
