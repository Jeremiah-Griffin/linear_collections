# Linear Collections
Collections backed by linear data structures.

Many advanced datastructures we learn in leactures and classes may yield incredible Big O algorithmic performance, but suffer from
high memory overhead or constant factors making them suboptimal for smaller collections. In many cases - especially considering the sensitivity of modern CPUs to cache-residency and missed branch predictions - maximizing the use of the stack
as well as well as minimizing fragmentation and branching can make simple linear datastructures to outperform their more theoretically performant brethren.

While the performance advantage in the large is typically not great, it may be more noticeable in latency-sensitive operations or within loops which allocate these
data structures in very large numbers.

# Motivation
The reasons for this crate's existence are twofold: 

1) To provide performant, branch-predictable, and simplistic collection types.
2) To provide types which uphold strong guarantees about their behavior with regards to dynamic allocation and minimize hidden control flow - especially panics.


For the first, the `try_` methods in `std`, while welcome, are inadequate for many systems as they do not provide type level guarantees of fallibility, nor of non-panicking behavior.

To provide peace of mind for such sentive applications this crate is aggressively feature-gated with defaults intended for high reliability and safety-critical applications for which panicking or even heap allocation is unacceptable.
For most users this excessively restrictive, and the `fallible` and `panicking` features (described below) are provided for types capable of heap allocation.

Though I personally *recommend* that all users default to the `fallible` types as they can reduce a small amount of binary bloat and aid program correctness on systems with sane OOM handling, they don't slot very well into existing code and often require API changes
which may not be teneble, especially for public APIs. For this reason, panicking variants - gated behind the "panicking" feature - are provided where possible.
 

# Feature Flags
- default: no features are enabled by default, and only a few types which are entirely stack allocated and non-panicking are exposed.
- "fallible": types which are guaranteed never to panic and return errors on memory allocation failure.
- "panicking": types allowed to panic on allocation failure or when their internal invariants are not upheld, emulating that behavior from `std`.
- "fallible_macros": compile type checking of fallible type literals.
- "panicking_macros": compile type checking of panicking type literals.
- "serde": ser/deserialization with serde.

# Changes for version 0.4.0
- Added iterator support
- Added FatVec, a vector type which can store a limited number of elements on the stack.
- Added FatMap & FatSet, map and set types backed by a FatVec.
- Added Fallible collection types which can gracefully return an error on a memory allocation failure. 
- Added VecDeque backed types
- Added StackList, a stack allocated vector-like type with a fixed capacity, but with a mutable number of elements.
- Added StackMap & StackList, map and set types backed by a StackList.
- Significantly expanded test coverage
- Much reorganization
- Many renames.
- Array backed types no longer implement LinearMap. An Array's fixed length means a lot of methods needed to be exluded from the traits, either to preserve their `const`ness, or because they needed to heap allocate. Removing these impls frees up the api and makes maintenance and testing of the remaining types easier.


# TODO:
- Serde support for array and StackList backed types via serde-big-array
- Fixed capactity, mutable length types.
- no_std support

# Stability Policy (pre - 1.0) 
- 1.0 will not be reached until all unstable features relied upon by this crate are either stabilized or removed.
- All changes up to and including removal of features prepended with "nightly_" are not considered breaking, so long as they are made 
  for conformance to the underlying unstable api or not otherwise specified as non-breaking. Please don't use them in public libraries.
