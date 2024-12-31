alias d := doc
alias l := nix-lint
alias uf := nix-update-flake-dependencies
alias uc := update-cargo-dependencies
#alias r := run
alias t := cargo-test
alias b := build
alias rr := run-release
alias cw := cargo-watch

default:
    @just --choose || brew install fzf || apt-get install fzf

clippy:
    cargo clippy --all-targets --all-features

nix-actionlint:
    nix develop .#actionlintShell --command actionlint

deny:
    cargo deny check

cargo-test:
    cargo test

nix-cargo-diet:
    nix develop .#lintShell --command cargo diet

nix-cargo-tarpaulin:
    nix develop .#lintShell --command cargo tarpaulin --out html --exclude-files "benches/*"

nix-cargo-public-api:
    nix develop .#lintShell --command cargo public-api

nix-cargo-diff:
    nix develop .#lintShell --command cargo public-api diff

nix-lint:
    nix develop .#lintShell --command cargo diet
    nix develop .#lintShell --command cargo deny check licenses sources
    nix develop .#lintShell --command typos
    nix develop .#lintShell --command lychee *.md
    nix develop .#fmtShell --command treefmt --fail-on-change
    nix develop .#lintShell --command cargo udeps
    nix develop .#lintShell --command cargo machete
    nix develop .#lintShell --command cargo outdated
    nix develop .#lintShell --command taplo lint
    nix develop .#actionlintShell --command actionlint --ignore SC2002
    cargo check --future-incompat-report
    nix flake check

build:
    cargo build

run-release:
    cargo run --release | 2>/dev/null

doc:
    cargo doc --open --offline

# Update and then commit the `Cargo.lock` file
update-cargo-dependencies:
    cargo update
    git add Cargo.lock
    git commit Cargo.lock -m "update(cargo): \`Cargo.lock\`"

# Future incompatibility report, run regularly
cargo-future:
    cargo check --future-incompat-report

nix-update-flake-dependencies:
    nix flake update --commit-lock-file

cargo-watch:
    cargo watch -x check -x test -x build


version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

bt := '0'

export RUST_BACKTRACE := bt

log := 'warn'

export RUST_LOG := log

# watch filesystem for changes and rerun tests
watch +ARGS='test':
	cargo watch --clear --exec '{{ARGS}}'

# show stats about torrents at `PATH`
stats PATH:
	cargo build --release
	time ./target/release/gnostr-modal --unstable torrent stats --input {{PATH}}

push:
	! git branch | grep '* master'
	git push github

# clean up feature branch BRANCH
done BRANCH=`git rev-parse --abbrev-ref HEAD`:
	git push github {{BRANCH}}:master
	git rebase github/master master
	git branch -d {{BRANCH}}

test:
	cargo test --all

# clippy:
# 	cargo clippy --all-targets --all-features

fmt:
	cargo +nightly fmt --all

forbid:
	./bin/forbid

preview-readme:
	grip -b README.md

# build and serve the book
book:
	mdbook serve book --open --dest-dir ../www/book || just dev-deps && just gen

dev-deps:
	brew install grip
	cargo install mdbook
	cargo install cargo-watch
	npm install --global asciicast2gif
	brew install imagemagick
	brew install gifsicle
	brew install asciinema

# update generated documentation
gen:
	cargo build
	cargo run --package gen -- --bin target/debug/gnostr-modal all

check-minimal-versions:
	./bin/check-minimal-versions

check: test clippy forbid check-minimal-versions gen
	git diff --no-ext-diff --quiet --exit-code
	cargo +nightly fmt --all -- --check

draft: push
	hub pull-request -o --draft

pr: check push
	hub pull-request -o

merge BRANCH=`git rev-parse --abbrev-ref HEAD`:
	#!/usr/bin/env bash
	set -euxo pipefail
	while ! hub ci-status --verbose {{BRANCH}}; do
		sleep 5
	done
	just done {{BRANCH}}

publish-check: check
	cargo outdated --exit-code 1
	grep '^\[{{version}}\]' target/gen/CHANGELOG.md

publish BRANCH=`git rev-parse --abbrev-ref HEAD`: publish-check (merge BRANCH)
	#!/usr/bin/env bash
	set -euxo pipefail
	git tag -a {{version}} -m 'Release {{version}}'
	git push github {{version}}
	while ! hub ci-status --verbose {{BRANCH}}; do
		sleep 5
	done
	cargo publish

# record, upload, and render demo animation
demo: demo-record demo-upload demo-render

demo-record:
	#!/usr/bin/env bash
	rm -rf tmp.torrent
	set -euxo pipefail
	cargo build --release --all
	asciinema rec \
		--title "gnostr-modal {{version}} Demo" \
		--command ./target/release/demo \
		--overwrite \
		tmp/demo.json

demo-upload:
	asciinema upload tmp/demo.json

demo-render:
	asciicast2gif -S4 tmp/demo.json www/demo.gif

# print commit metadata types
commit-types:
	cargo run --package gen -- --bin target/debug/gnostr-modal commit-types

# open site index
www:
	open www/index.html

# retrieve large collection of torrents from the Internet Archive
get-torrents:
	aria2c \
		-d dat \
		-x 10 \
		'https://ia802701.us.archive.org/21/items/2014_torrent_archive_organized/torrent_archive_organized.zip'

# download bittorrent.org repository
get-beps:
	git clone git@github.com:bittorrent/bittorrent.org.git tmp/bittorrent.org

build-image:
  podman build -t gnostr-modal .
  podman run gnostr-modal

test-release:
  -git tag -d test-release
  -git push origin :test-release
  git tag test-release
  git push origin test-release

outdated:
  cargo outdated --workspace --root-deps-only

unused:
  cargo +nightly udeps --workspace

coverage:
  cargo llvm-cov --html
  open target/llvm-cov/html/index.html

update-changelog:
  echo >> CHANGELOG.md
  git log --pretty='format:- %s' >> CHANGELOG.md

update-contributors:
  cargo run --release --package update-contributors

# build all examples
nix-examples:
    nix develop --command $SHELL
    example_list=$(cargo build --example 2>&1 | sed '1,2d' | awk '{print $1}')

    # Build each example
    # shellcheck disable=SC2068
    for example in ${example_list[@]}; do
    cargo build --example "$example"
    done

nix-examples-msrv:
    set -x
    nix develop .#msrvShell --command
    rustc --version
    cargo --version
    example_list=$(cargo build --example 2>&1 | grep -v ":")

    # Build each example
    # shellcheck disable=SC2068
    for example in ${example_list[@]}; do
    cargo build --example "$example"
    done


