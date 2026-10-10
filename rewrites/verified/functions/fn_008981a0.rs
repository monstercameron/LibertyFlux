// original: 0x008981A0 input_store_byte_0xe6

/// Stores the low byte of the argument at +0xE6 of the object `this`. No table is read.
/// Thiscall with one stack argument, callee cleans up.
lf_checker_rt::export!(thiscall, rw_008981a0(this: u32, value: u32) -> () {
    unsafe {
        const FIELD: u32 = 0xE6;
        (this.wrapping_add(FIELD) as *mut u8).write(value as u8);
    }
});
