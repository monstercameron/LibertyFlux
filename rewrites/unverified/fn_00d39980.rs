// original: 0x00D39980 CDummyTask_Flee::vf4

/// Set the cached condition byte at +0x3e to one only when the object at +0x10 exists and its state word at +0x6c is nonzero; otherwise clear the byte. ECX is `this`; the function has no stack arguments and its return register is not meaningful.
lf_checker_rt::export!(thiscall, rw_00d39980(this: u32) -> u32 {
    unsafe {
        const RELATED_OBJECT: u32 = 0x10;
        const RELATED_STATE: u32 = 0x6c;
        const CACHED_CONDITION: u32 = 0x3e;
        let related = (this.wrapping_add(RELATED_OBJECT) as *const u32).read_unaligned();
        let enabled = related != 0
            && (related.wrapping_add(RELATED_STATE) as *const u32).read_unaligned() != 0;
        (this.wrapping_add(CACHED_CONDITION) as *mut u8).write(u8::from(enabled));
        0
    }
});
