[1mdiff --cc Cargo.lock[m
[1mindex ffb0882,f3d71c9..0000000[m
[1m--- a/Cargo.lock[m
[1m+++ b/Cargo.lock[m
[36m@@@ -44,13 -51,23 +59,25 @@@[m [mversion = "1.0.15[m
  source = "registry+https://github.com/rust-lang/crates.io-index"[m
  checksum = "4a5f13b858c8d314ee3e8f639011f7ccefe71f97f96e50151fb991f267928e2c"[m
  [m
[32m+ [[package]][m
[32m+ name = "kani-verifier"[m
[32m+ version = "0.62.0"[m
[32m+ source = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+ checksum = "f16df5f75a0f9f3b22f3dd270847c3df14a53a7d50ddbee9fa7a1b102785b6a0"[m
[32m+ dependencies = [[m
[32m+  "anyhow",[m
[32m+  "home",[m
[32m+  "os_info",[m
[32m+ ][m
[32m+ [m
  [[package]][m
  name = "linear_collections"[m
[31m- version = "0.4.0"[m
[32m+ version = "0.3.1"[m
  dependencies = [[m
[32m + "fallible_linear_collections_macros",[m
[32m+  "kani-verifier",[m
   "linear_collections_macros",[m
[32m + "panicking_linear_collections_macros",[m
   "serde",[m
   "serde_test",[m
   "trybuild",[m
[36m@@@ -70,19 -93,21 +103,29 @@@[m [mversion = "2.7.4[m
  source = "registry+https://github.com/rust-lang/crates.io-index"[m
  checksum = "78ca9ab1a0babb1e7d5695e3530886289c18cf2f87ec19a575a0abdce112e3a3"[m
  [m
[32m+ [[package]][m
[32m+ name = "os_info"[m
[32m+ version = "3.11.0"[m
[32m+ source = "registry+https://github.com/rust-lang/crates.io-index"[m
[32m+ checksum = "41fc863e2ca13dc2d5c34fb22ea4a588248ac14db929616ba65c45f21744b1e9"[m
[32m+ dependencies = [[m
[32m+  "log",[m
[32m+  "windows-sys 0.52.0",[m
[32m+ ][m
[32m+ [m
[32m +[[package]][m
[32m +name = "panicking_linear_collections_macros"[m
[32m +version = "0.4.0"[m
[32m +dependencies = [[m
[32m + "quote",[m
[32m + "syn",[m
[32m +][m
[32m +[m
  [[package]][m
  name = "proc-macro2"[m
[31m- version = "1.0.94"[m
[32m+ version = "1.0.95"[m
  source = "registry+https://github.com/rust-lang/crates.io-index"[m
[31m- checksum = "a31971752e70b8b2686d7e46ec17fb38dad4051d94024c88df49b667caea9c84"[m
[32m+ checksum = "02b3e5e68a3a1a02aad3ec490a98007cbc13c37cbe84a3cd7b8e406d76e7f778"[m
  dependencies = [[m
   "unicode-ident",[m
  ][m
[1mdiff --cc Cargo.toml[m
[1mindex 9f5949b,62e31af..0000000[m
[1m--- a/Cargo.toml[m
[1m+++ b/Cargo.toml[m
[36m@@@ -1,4 -1,6 +1,3 @@@[m
[31m -[workspace][m
[31m -members = ["macros"][m
[31m--[m
  [package][m
  name = "linear_collections"[m
  #rember to update changelog whenever [m
[36m@@@ -24,19 -20,10 +23,20 @@@[m [mpanicking_linear_collections_macros = {[m
  [m
  [dev-dependencies][m
  trybuild = {version = "1.0.96"}[m
[32m+ kani-verifier = "0.62.0"[m
  [m
  [features][m
[32m +#panicking and nightly_fallible are seperate in case a create wants [m
[32m +#only fallible collections. Choosing no default features and nightly_fallible will achieve this.[m
  default = [][m
[32m +[m
[32m +[m
[32m +#Types which handle allocation failure by panicking.[m
[32m +panicking = [][m
[32m +#Types which handle allocation failure by returning an error. [m
[32m +nightly_fallible = ["nightly"][m
[32m +#Performance optimizations and aditional functionality available only on nightly rust.[m
[32m +nightly = [][m
  #Adds support for ser/deserialization of *non-array types* with serde.[m
  #To serialize array backed types, enable the "serde-big-array" feature as well.[m
  serde = ["dep:serde", "dep:serde_test"][m
[1mdiff --cc justfile[m
[1mindex bb6b77d,68838ad..0000000[m
[1m--- a/justfile[m
[1m+++ b/justfile[m
[36m@@@ -1,25 -1,37 +1,45 @@@[m
[31m- set windows-shell := ["powershell.exe", "-NoLogo", "-Command"][m
[32m+ current_branch := `git branch --show-current`[m
  [m
[31m- setup:[m
[31m- 	cargo install cargo-hack[m
[31m- 	rustup +nightly component add miri[m
[32m +[m
  default:[m
  	just -l[m
  [m
[31m- #Check build with all features enabled.[m
[32m +alias c := check[m
[32m +check:[m
[32m +	cargo hack check --feature-powerset --no-dev-deps  --group-features nightly_fallible,fallible_macros --group-features panicking,panicking_macros[m
[32m +[m
[32m+ commit MESSAGE: pre_commit[m
[32m+ 	git commit -a --message {{MESSAGE}}[m
[32m+ [m
[32m+ #runs before all commits[m
[32m+ [private][m
[32m+ pre_commit:[m
[32m+ 	echo "committing to {{current_branch}}..."[m
[32m+ 	just {{ if current_branch == "trunk" {"prepare_master_commit"} else {"prepare_other_commit"} }}[m
[32m+ [m
[32m+ [private][m
[32m+ prepare_master_commit:[m
[32m+ 	just check[m
[32m+ 	just test[m
[32m+ [m
[32m+ [private][m
[32m+ prepare_other_commit:[m
[32m+ [m
[32m+ setup:[m
[32m+ 	cargo add cargo-expand[m
[32m+ 	rustup component add +nightly miri[m
[32m+ 	cargo add cargo-hack[m
[32m+ 	cargo install --locked kani-verifier[m
[32m+ 	cargo kani setup[m
[32m+ [m
  [m
[31m- #Runs all tests specified by FILTER. If not provided, runs all tests in the crate.[m
  alias t := test[m
[31m -test:[m
[31m -	cargo t --release --all-features[m
[32m +test PATTERN = "":[m
[32m +	#test once with all features to hit the trybuild macro tests which dont work under miri.[m
[32m +	cargo test {{PATTERN}} --all-features[m
[32m +	cargo hack miri test --feature-powerset --no-dev-deps {{PATTERN}} --group-features nightly_fallible,fallible_macros --group-features panicking,panicking_macros[m
[32m +[m
  [m
[31m- publish: test[m
[31m- 	cargo publish[m
[32m+ update:[m
[32m+ 	cargo update[m
[32m+ 	cd macros && cargo update[m
[1mdiff --git a/macros/src/lib.rs b/macros/src/lib.rs[m
[1mindex 36f31c4..d754ec5 100644[m
[1m--- a/macros/src/lib.rs[m
[1m+++ b/macros/src/lib.rs[m
[36m@@ -1,7 +1,7 @@[m
 use std::collections::BTreeSet;[m
 [m
 use proc_macro::TokenStream;[m
[31m-use quote::quote;[m
[32m+[m[32muse quote::{quote, ToTokens};[m
 use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, ExprTuple, Token};[m
 [m
 struct MapLiteral {[m
[36m@@ -31,7 +31,7 @@[m [mfn validate_map_literal(input: &MapLiteral) {[m
     let mut keys: BTreeSet<String> = BTreeSet::new();[m
 [m
     for e in elements.iter() {[m
[31m-        let element_string = e.to_token_stream().to_string();[m
[32m+[m[32m        let element_string = e.into_token_stream().to_string();[m
 [m
         let tuple_pair_count = e.elems.len();[m
 [m
