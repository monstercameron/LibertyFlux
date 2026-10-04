// original: 0x00cf4830 climb_effect_probe3 (proposed)

/// Resolves a three-word probe vector for a climb effect object: queries the
/// object's virtual probe (slot 0xa0); when it reports null the fallback
/// selector at `+0x100` is used, otherwise the probe is queried again and its
/// result's selector (slot 0xe0) is used. The word at `+4` of the selector
/// and 0 index the shared table; a non-negative index reads the vector from
/// the table entry's block at `+0x30`, otherwise from the fallback block at
/// `+0x20` of the object. Returns the output pointer.
///
/// Original: 0x00cf4830 (stdcall, two stack words: out, object).
lf_checker_rt::export!(stdcall, rw_00cf4830(out: u32, obj: u32) -> u32 {
    unsafe {
        const PROBE_SLOT: u32 = 0xa0;
        const SELECTOR_SLOT: u32 = 0xe0;
        const TABLE_CALLEE: u32 = 3;
        const ENTRY_CALLEE: u32 = 4;
        type VProbe = extern "thiscall" fn(u32) -> u32;
        let vt = (obj as *const u32).read_unaligned();
        let probe: VProbe =
            core::mem::transmute(((vt + PROBE_SLOT) as *const u32).read_unaligned() as usize);
        let first = probe(obj);
        let selector = if first == 0 {
            ((obj + 0x100) as *const u32).read_unaligned()
        } else {
            let vt2 = (obj as *const u32).read_unaligned();
            let probe2: VProbe = core::mem::transmute(
                ((vt2 + PROBE_SLOT) as *const u32).read_unaligned() as usize);
            let mid = probe2(obj);
            let vt3 = (mid as *const u32).read_unaligned();
            let select: VProbe = core::mem::transmute(
                ((vt3 + SELECTOR_SLOT) as *const u32).read_unaligned() as usize);
            select(mid)
        };
        let key = ((selector + 4) as *const u32).read_unaligned();
        let index = lf_checker_rt::callee_cdecl!(TABLE_CALLEE, u32, key, 0);
        let block = if (index as i32) >= 0 {
            let entry = lf_checker_rt::callee_thiscall!(ENTRY_CALLEE, u32, obj, index);
            entry.wrapping_add(0x30)
        } else {
            ((obj + 0x20) as *const u32).read_unaligned().wrapping_add(0x30)
        };
        for i in 0..3u32 {
            let v = ((block + i * 4) as *const u32).read_unaligned();
            ((out + i * 4) as *mut u32).write_unaligned(v);
        }
        out
    }
});
