// original: 0x006221a0 net_entry_copy_tail
/// Copy an entry's header through the shared helper, then its tail.
///
/// Copies the head of `src` into `this` via the shared entry-copy helper,
/// then copies the word at +0x290 and the 0x80-dword block at +0x298.
/// Returns `this`.
export!(thiscall, rw_006221a0(this: u32, src: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this, src);
        let dst = this as *mut u8;
        let s = src as *const u8;
        (dst.add(0x290) as *mut u32)
            .write_unaligned((s.add(0x290) as *const u32).read_unaligned());
        core::ptr::copy_nonoverlapping(
            (s.add(0x298)) as *const u32,
            (dst.add(0x298)) as *mut u32,
            0x80,
        );
        this
    }
});
