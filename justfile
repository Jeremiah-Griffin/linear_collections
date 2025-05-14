current_branch := `git branch --show-current`


default:
	just -l

merge TO_MERGE:
	echo "merging {{TO_MERGE}} into {{current_branch}}..."
	fossil addremove --dotfiles
	git merge "{{TO_MERGE}}"
	git branch --delete "{{TO_MERGE}}"
	#mostly to ensure that the branch actually gets integrated and closed.
	just commit
	echo "merged {{TO_MERGE}} into {{current_branch}}"	

alias c := check
check:
	cargo hack check --feature-powerset --no-dev-deps  --group-features nightly_fallible,fallible_macros --group-features panicking,panicking_macros

commit MESSAGE:
	just pre_commit
	git commit  --message "{{MESSAGE}}" -a
	git sync
	git push

#runs before all commits
[private]
pre_commit:
	echo "committing to {{current_branch}}..."
	just {{ if current_branch == "trunk" {"prepare_master_commit"} else {"prepare_other_commit"} }}

[private]
prepare_master_commit:
	just check
	just test

[private]
prepare_other_commit:

setup:
	cargo add cargo-expand
	rustup component add +nightly miri
	cargo add cargo-hack
	cargo install --locked kani-verifier
	cargo kani setup


alias t := test
test PATTERN = "":
	#test once with all features to hit the trybuild macro tests which dont work under miri.
	cargo test {{PATTERN}} --all-features
	cargo hack miri test --feature-powerset --no-dev-deps {{PATTERN}} --group-features nightly_fallible,fallible_macros --group-features panicking,panicking_macros


update:
	cargo update
	cd macros && cargo update
