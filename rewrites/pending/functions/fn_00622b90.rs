// original: 0x00622b90 net_entry_init_zero_tail
/// Initialise an entry through the shared helper, then zero its tail.
///
/// Runs the shared entry initialiser on `this`, then zeroes the 0x200-byte
/// tail at `this+0x298`. Returns `this`.
export!(thiscall, rw_00622b90(this: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this);
        core::ptr::write_bytes((this as *mut u8).add(0x298), 0, 0x200);
        this
    }
});
