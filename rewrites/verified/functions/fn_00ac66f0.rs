// original: 0x00AC66F0 stream_gather_indexed (proposed)

/// Gather table words through the object's index vector.
///
/// The original fetches the index vector and element count through the
/// object's virtual slots `+0x14`/`+0x18` (cdecl, three pointers: `obj`,
/// `out`, `table`), then copies `table[u16[idx[i]]]` to `out[i]` for each
/// of the low 16 bits of the count. It returns the last word copied, or the
/// count answer when the count is zero.
lf_checker_rt::export!(cdecl, rw_00AC66F0(obj: u32, out: u32, table: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const COUNT: u32 = 2;
        const FETCH_SLOT: u32 = 0x14;
        const COUNT_SLOT: u32 = 0x18;
        let vt = (obj as *const u32).read_unaligned();
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vt.wrapping_add(FETCH_SLOT) as *const u32).read_unaligned() as usize);
        let idx = fetch(obj);
        let vt2 = (obj as *const u32).read_unaligned();
        let count: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vt2.wrapping_add(COUNT_SLOT) as *const u32).read_unaligned() as usize);
        let answer = count(obj);
        let n = answer & 0xffff;
        if n == 0 {
            return answer;
        }
        let mut last = 0u32;
        for i in 0..n {
            let at = (idx.wrapping_add(i * 2) as *const u16).read_unaligned() as u32;
            last = (table.wrapping_add(at * 4) as *const u32).read_unaligned();
            (out.wrapping_add(i * 4) as *mut u32).write_unaligned(last);
        }
        let _ = (FETCH, COUNT);
        last
    }
});
