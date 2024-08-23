set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default:
	just -l

alias c := check
check:
	cargo check --all-features

alias t := test
test:
	cargo t --release --all-features

publish: test
	cargo publish

	