
IS_TARPAULIN:=$(shell cargo list | grep tarpaulin)
IS_CI:=$(CI)
IS_SCCACHE:=""

.DEFAULT_GOAL := help
help:             ## Show available options with this Makefile
	@grep -F -h "##" $(MAKEFILE_LIST) | grep -v grep | awk 'BEGIN { FS = ":.*?##" }; { printf "%-18s  %s\n", $$1,$$2 }'

.PHONY: install_sccache
install_sccache: ## Check if sccache is installed, else install it.
ifdef IS_CI
	@IS_SCCACHE=$(shell ls $(CI_PROJECT_DIR)/.cargo/bin/ | grep sccache)
else
	@IS_SCCACHE=$(shell sh -c 'type -P sccache')
endif

ifndef IS_SCCACHE
	@echo "*********** Installing sccache since its not found in the path    ***********"
	@cargo install sccache
else
	@echo "*********** Found sccache in the path. Expect faster builds/tests ***********"
endif

.PHONY: install_tarpaulin
install_tarpaulin: ## Check if tarpaulin is installed, else install it.
ifndef IS_TARPAULIN
	@echo "*************** Installing tarpaulin as its not found in the path ***********"
	@RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo +nightly install cargo-tarpaulin ;
endif

.PHONY : test-cover
test-cover: install_tarpaulin install_sccache       ## Run all the tests with coverage
ifdef IS_CI
	@echo "*********** Running coverage command on CI                        ***********"
	@RUSTC_WRAPPER=$(CI_PROJECT_DIR)/.cargo/bin/sccache cargo +nightly tarpaulin --root $(CI_PROJECT_DIR) --exclude-files *.cargo/** --verbose
else
	@echo "*********** This is Local server. Running coverage now            ***********"
	@RUSTC_WRAPPER=$(HOME)/.cargo/bin/sccache cargo +nightly tarpaulin --verbose
endif

.PHONY : test
test:   install_sccache       ## Run all the tests
ifdef IS_CI
	@echo "*********** This is CI server. Running tests now                  ***********"
	@RUSTC_WRAPPER=$(CI_PROJECT_DIR)/.cargo/bin/sccache cargo test --all -- --nocapture
else
	@echo "*********** This is Local Machine. Running tests now              ***********"
	@RUSTC_WRAPPER=$(HOME)/.cargo/bin/sccache cargo test --all -- --nocapture
endif

clean:         ## Clean the application
	@cargo clean

.PHONY : build_release
build_release: clean test install_sccache ## Create a release build
	RUSTC_WRAPPER=$(HOME)/.cargo/bin/sccache RUSTFLAGS='-C link-args=-s' cargo build --release --target=x86_64-unknown-linux-musl

.PHONY : lint
lint: ## Run tests, fmt and clippy on this
	touch src/lib.rs && cargo clippy --all && cargo fmt --all 

.PHONY : docs
docs:  ## Generate the docs for this project. Docs are located in target/doc/test_rs
	@cargo doc --no-deps

.PHONY : docs-open
docs-open: docs ## Generate docs and open with xdg-open
	@xdg-open target/doc/aiven_rs/index.html
