// original: 0x00897F90 input_store_float_0xc8

/// Stores a float argument, as its bits, at +0xC8 of the object `this`. No table is read.
/// Thiscall with one stack argument, callee cleans up.
lf_checker_rt::export!(thiscall, rw_00897f90(this: u32, value: u32) -> () {
    unsafe {
        const FIELD: u32 = 0xC8;
        (this.wrapping_add(FIELD) as *mut u32).write_unaligned(value);
    }
});
