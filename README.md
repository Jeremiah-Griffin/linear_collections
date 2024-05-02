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

- A macro based alternative for ArrayMap::new_unchecked which will statically guarantee that array literals have no duplicate keys
- Fallible allocating apis 
- VecDeque backed types (nice if you need a vecdeque to be returned from into_inner())
- Serde support for Array types via serde-big-array (?)

