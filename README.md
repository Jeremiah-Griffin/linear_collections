# Linear Collections
Map and Set types backed by linear data structures.

Many use cases of maps are for small collections where the penalties of a comparable data structure from std 
(memory overhead, hashing throughput, indirection and potential cache dislocality of a nonlinear data structure)
are greater than their comparatively reduced algorithmic complexity.

This penalty usually is not great, however, a (small) amount of performance may be gained if allocating 
small such structures in a tight loop - say, deserializing the claims from a Json Web Token.

# Motivation
There exists another crate implementing linear collection types (linear_map)
however, it only supports collection backed by a vector: I personally needed Array types as well.

# Feature Flags
- "macros": compile type checking of type literals.
- "serde": ser/deserialization with serde.

# Changes for version 0.3.0
- fixed merge_from_iter
- added macros
- added many trait derives
- added serde support for VecSet
- ArrayMap::new_unchecked renamed to ArrayMap::from_array_unchecked
- added VecSet::from_map_unchecked 

# TODO:
- Fallible allocating apis 
- VecDeque backed types (nice if you need a vecdeque to be returned from into_inner())
- Serde support for Array types via serde-big-array
- Iterator Support
- More tests


# Stability Policy (pre - 1.0) 
- 1.0 will not be reached until all unstable features relied upon by this crate are either stabilized or removed.
- All changes up to and including removal of features prepended with "nightly" are not considered breaking, so long as they are made 
  for conformance to the underlying unstable api or not otherwise specified as non-breaking. Please don't use them in public libraries.

