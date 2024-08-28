set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

setup:
	cargo install cargo-hack
	rustup +nightly component add miri

default:
	just -l

#Check build with all features enabled.
alias c := check
check:
	cargo hack check --feature-powerset --no-dev-deps  --group-features nightly_fallible,fallible_macros --group-features panicking,panicking_macros


#Runs all tests specified by FILTER. If not provided, runs all tests in the crate.
alias t := test
test *FILTER:
	cargo hack miri test --feature-powerset --no-dev-deps {{FILTER}} --group-features nightly_fallible,fallible_macros --group-features panicking,panicking_macros


publish: test
	cargo publish
