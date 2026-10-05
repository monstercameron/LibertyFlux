// original: 0x009DC5D0 CPedModelInfo::vf14

/// Return the byte at `index + this + 0x134`, zero-extended.
///
/// A virtual getter (slot 14) that reads one flag byte out of a table stored
/// at a fixed offset past the object, indexed by the caller's argument.
///
/// Original: 0x009DC5D0 (thiscall, one stack argument, no outgoing calls).
lf_checker_rt::export!(thiscall, rw_009DC5D0(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x134;

        let addr = index.wrapping_add(this).wrapping_add(TABLE_OFF);
        (addr as *const u8).read() as u32
    }
});
