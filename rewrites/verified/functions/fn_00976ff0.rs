// original: 0x00976ff0 audio_nibble_dispatch (proposed)

/// Dispatch on a nibble of the flags word: null gives 0; bits 6..9 equal to
/// 2 read the cached pointer at +0xC30, equal to 3 run the loader callee on
/// the sub-object at +0x3C0 and return its result, anything else gives 0.
/// Equality compares only.
/// Original: 0x00976FF0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00976ff0(obj: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x28;
        const CACHED: u32 = 0xC30;
        const SUB: u32 = 0x3C0;
        const LOAD: u32 = 1;
        if obj == 0 {
            return 0;
        }
        let sel = ((((obj.wrapping_add(FLAGS)) as *const u32).read_unaligned() >> 6) & 0xF)
            .wrapping_sub(2);
        if sel == 0 {
            return ((obj.wrapping_add(CACHED)) as *const u32).read_unaligned();
        }
        if sel.wrapping_sub(1) != 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(LOAD, u32, obj.wrapping_add(SUB))
    }
});
