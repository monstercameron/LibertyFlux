// original: 0x0093f140 stream_selected_apply (proposed)

/// Run the slot hook over `buf` for the selected object.
///
/// A null selected object zeroes the three buffer words. Otherwise asks
/// the flagged-word getter twice: when the first answer is null the hook
/// (virtual slot `HOOK_SLOT`) runs with the selected object, else the
/// second answer's hook runs with that object instead. Returns `buf`. The
/// hook is contract callee 3, reached through the object's own table.
///
/// Original: 0x0093f140 (cdecl, one stack word; two direct + one virtual callee).
lf_checker_rt::export!(cdecl, rw_0093f140(buf: u32) -> u32 {
    const SELECTED_GETTER: u32 = 1;
    const FLAGGED_GETTER: u32 = 2;
    const HOOK_SLOT: u32 = 0xEC;
    unsafe {
        let obj: u32 = lf_checker_rt::callee_cdecl!(SELECTED_GETTER, u32,);
        if obj == 0 {
            for off in [0u32, 4, 8] {
                (buf.wrapping_add(off) as *mut u32).write_unaligned(0);
            }
            return buf;
        }
        let hook_this: u32 = {
            let first: u32 = lf_checker_rt::callee_cdecl!(FLAGGED_GETTER, u32, 0u32);
            if first == 0 {
                obj
            } else {
                lf_checker_rt::callee_cdecl!(FLAGGED_GETTER, u32, 0u32)
            }
        };
        let hook: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((((hook_this as *const u32).read_unaligned()) + HOOK_SLOT) as *const u32)
                .read_unaligned() as usize,
        );
        hook(hook_this, buf);
        buf
    }
});
