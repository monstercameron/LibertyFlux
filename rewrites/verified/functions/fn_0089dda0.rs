// original: 0x0089dda0 audio_slot_release_forward (proposed)

/// Forwards a release to the object's virtual slot 0x14 with argument 0.
///
/// Loads the virtual table from `obj`, calls through slot `0x14`
/// (thiscall/1 with 0), intercepted by planting the stub address in the
/// fabricated object, and returns the answer.
///
/// Original: 0x0089dda0 (cdecl, one stack word; 12 bytes).
lf_checker_rt::export!(cdecl, rw_0089dda0(obj: u32) -> u32 {
    unsafe {
        const VT_SLOT: usize = 0x14;
        let vt = (obj as *const u32).read_unaligned();
        let tgt = ((vt as *const u8).byte_add(VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        f(obj, 0)
    }
});
