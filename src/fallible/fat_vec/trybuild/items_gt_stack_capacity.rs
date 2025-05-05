use linear_collections::fallible::FatVec;
fn main() {
    let _fv = FatVec::<u8, 10>::with_partial_array([0; 11]);
}
