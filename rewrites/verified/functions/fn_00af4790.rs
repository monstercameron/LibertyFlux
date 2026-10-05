// original: 0x00AF4790 drawable_notify_if_set (proposed)

/// Notify the helper at slot 0x1C when present and the flag word is zero.
///
/// Loads the helper object at `this+4`; when it is null, or when `flag`
/// is nonzero, returns at once. Otherwise calls the helper's function
/// table slot 0x1C with the helper as object. Returns nothing meaningful.
///
/// Original: 0x00AF4790 (thiscall, one stack word, one table-indirect callee).
lf_checker_rt::export!(thiscall, rw_00af4790(this: u32, flag: u32) -> () {
    unsafe {
        const HELPER_OFF: u32 = 4;
        const SLOT: u32 = 0x1C;
        const NOTIFY_CALLEE: u32 = 1;
        let _ = NOTIFY_CALLEE;
        let obj = ((this.wrapping_add(HELPER_OFF)) as *const u32).read_unaligned();
        if obj == 0 || flag != 0 {
            return;
        }
        let vt = ((obj) as *const u32).read_unaligned();
        let tgt = ((vt.wrapping_add(SLOT)) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
        f(obj);
    }
});
