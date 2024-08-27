set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

setup:
	cargo install cargo-hack

default:
	just -l

#Check build with all features enabled.
alias c := check
check:
	cargo check --all-features

#Runs all tests specified by FILTER. If not provided, runs all tests in the crate.
alias t := test
test *FILTER:
	cargo hack test --each-feature {{FILTER}}


publish: test
	cargo publish
