DESTDIR =
PREFIX  = /usr/local

all: target/release/grlm
build: target/release/grlm

target/release/grlm:
	cargo build --release

install: install-grlm

install-grlm: target/release/grlm
	install -m755 -- target/release/grlm "$(DESTDIR)$(PREFIX)/bin/"

test: target/release/grlm
	cargo test --release $(CARGO_OPTS)

check: test

uninstall:
	-rm -f -- "$(DESTDIR)$(PREFIX)/bin/grlm"

clean:
	cargo clean

help:
	@echo 'Available make targets:'
	@echo '  all         - build grlm (default)'
	@echo '  build       - build grlm'
	@echo '  clean       - run `cargo clean`'
	@echo '  install     - build and install grlm'
	@echo '  install-grlm - build and install grlm'
	@echo '  test        - run `cargo test`'
	@echo '  uninstall   - uninstall fish, manpage, and completions'
	@echo '  help        - print this help'
	@echo
	@echo
	@echo 'Variables:'
	@echo '  DESTDIR  - A path that'\''s prepended to installation paths (default: "")'
	@echo '  PREFIX   - The installation prefix for everything except zsh completions (default: /usr/local)'
	@echo '  FEATURES - The cargo feature flags to use. Set to an empty string to disable git support'

.PHONY: all build target/release/grlm install-grlm \
	clean uninstall help
