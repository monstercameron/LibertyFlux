// original: 0x00AF47B0 drawable_sync_if_set (proposed)

/// Call the helper's slot 0x10 when a helper is attached, else return.
///
/// Loads the helper object at `this+4`; when null returns at once. The
/// stack word is popped but never read. Returns nothing meaningful.
///
/// Original: 0x00AF47B0 (thiscall, one unread stack word, one table-indirect callee).
lf_checker_rt::export!(thiscall, rw_00af47b0(this: u32, _u: u32) -> () {
    unsafe {
        const HELPER_OFF: u32 = 4;
        const SLOT: u32 = 0x10;
        let obj = ((this.wrapping_add(HELPER_OFF)) as *const u32).read_unaligned();
        if obj == 0 {
            return;
        }
        let vt = ((obj) as *const u32).read_unaligned();
        let tgt = ((vt.wrapping_add(SLOT)) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
        f(obj);
    }
});
