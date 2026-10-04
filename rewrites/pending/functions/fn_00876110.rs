// original: 0x00876110 crmt_extrapolate_node_store_rate
/// Store a rate sample into an extrapolate-node accumulator.
///
/// thiscall/1. Copies the 32-bit argument to the accumulator field; the
/// copy is bitwise, so every NaN payload survives unchanged.
export!(thiscall, rw_00876110(this: *mut u8, sample_bits: u32) -> () {
    unsafe {
        *(this.add(0x30) as *mut u32) = sample_bits;
    }
});
