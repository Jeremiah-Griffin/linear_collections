# Linear Collections
Map and Set types backed by linear data structures.

Many advanced datastructures we learn in leactures and classes may yield incredible Big O algorithmic performance, but suffer from
excessive memory usage or high constant factors. In many cases - especially considering the sensitivity of modern CPUs to cache-residency and missed branch predictions - maximizing the use of the stack
as well as well as minimizing fragmentation and branching can render simple linear datastructures and alogorithms to outperform their more theoritically perofrmant brethren.

While the performance advantage in the large is typically not great, it may be more noticeable in latency-sensitive operations or within loops which allocate these
data structures in very large numbers.

# Motivation
There exists another crate implementing linear collection types (linear_map)
however, it only supports collection backed by a vector: I personally needed Array types as well.

# Feature Flags
- "fallible_macros": compile type checking of fallible type literals.
- "panicking_macros": compile type checking of panicking type literals.
- "serde": ser/deserialization with serde.

# Changes for version 0.4.0
- Added iterator support
- Added FatVec/Map/Set, a vector type which can store a limited number of elements on the stack.
- Added Fallible collection types which can gracefully return an error on a memory allocation failure. 
- Added VecDeqe backed types
- Reorganization & Renames
- Array backed types no longer implement LinearMap. Array's fixed length means a lot of methods needed to be exluded from the traits, either to preserve their `const`ness, or because they needed to heap allocate. Removing these impls frees up the api and makes maintenance and testing of the remaining types easier.


# TODO:
- Serde support for Array types via serde-big-array
- Fixed capactity, mutable length types.


# Stability Policy (pre - 1.0) 
- 1.0 will not be reached until all unstable features relied upon by this crate are either stabilized or removed.
- All changes up to and including removal of features prepended with "nightly" are not considered breaking, so long as they are made 
  for conformance to the underlying unstable api or not otherwise specified as non-breaking. Please don't use them in public libraries.

