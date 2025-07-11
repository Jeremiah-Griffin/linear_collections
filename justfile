set quiet

current_branch := `git branch --show-current`

default:
	just -l

branch BRANCH_NAME:
	git branch "{{BRANCH_NAME}}"
	git checkout "{{BRANCH_NAME}}"
	#this always fails to commit due to the working tree being clean. I don't use git and will fix this later
	just commit "created {{BRANCH_NAME}}"


alias c := check
check:
	cargo hack check --feature-powerset --no-dev-deps 

commit MESSAGE:
	echo "committing to {{current_branch}}..."
	just pre_commit
	just commit_inner "{{MESSAGE}}"

[private]
commit_inner MESSAGE:
	git commit  --message "{{MESSAGE}}" -a
	git push --all -u

[private]
pre_commit:
	git add --all
	git fetch --all
	just {{ if current_branch == "trunk" {"prepare_master_commit"} else {"prepare_other_commit"} }}

[private]
prepare_master_commit:
	just check
	just test

[private]
prepare_other_commit:

merge TO_MERGE:
	just pre_commit
	echo "merging {{TO_MERGE}} into {{current_branch}}..."
	git merge "{{TO_MERGE}}"
	git branch --delete "{{TO_MERGE}}"
	#mostly to ensure that the branch actually gets integrated and closed.
	just commit_inner "merged {{TO_MERGE}} into {{current_branch}}"
	echo "Merge complete."

rebuild:
	cargo clean
	cargo build
	
setup:
	cargo install cargo-expand
	cargo install --locked kani-verifier
	cargo kani setup


alias t := test
test PATTERN = "":
	cargo test {{PATTERN}} --all-features
	#--randomize-layout randomizes struct layout, ensuring we don't rely on a unstable ordering in unsafe
	#--force-build runs a fresh incremental compile
	#-j enables running harnesses multi core
	#--output-format-terse is required by -j
	cargo kani --randomize-layout --harness "{{PATTERN}}" --force-build -Z unstable-options -Z loop-contracts -Z concrete-playback -j --output-format=terse
	

update:
	cargo update
	cd macros && cargo update

