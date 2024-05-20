# Linear Collections
Map and Set types backed by linear data structures.

Many use cases of maps are for small collections where the penalties of a comparable data structure from std 
(memory overhead, hashing throughput, indirection and potential cache dislocality of a nonlinear data structure)
are greater than their comparatively reduced algorithmic complexity.

This penalty usually is not great, however, a (small) amount of performance may be gained if allocating 
small such structures in a tight loop - say, deserializing the claims from a Json Web Token.

# Motivation
There exists another crate implementing linear collections (linear_map)
however, it only supports collection backed by a vector: I personally needed Array types as well.

# Future plans
I am likely going to add VecDeque types and will look into how serde supports fixed length collection types.
They aren't in now as I don't immediately need them, but will be added:

- Fallible allocating apis 
- VecDeque backed types (nice if you need a vecdeque to be returned from into_inner())
- Serde support for Array types via serde-big-array (?)


# Stability Policy (pre - 1.0) 
- 1.0 will not be reached until all unstable features relied upon by this crate are either stabilized or removed.
- All changes up to and including removal of features prepended with "nightly" (i.e. nightly_vecdeque) are not considered breaking, so long as they are made 
  for conformance to the underlying unstable api.
- Soundness bug fixes are not considered to be breaking.
