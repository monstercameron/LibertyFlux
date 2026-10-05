// original: 0x00c82470 scenario_batch_emit (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `frame_pointer_args` (plus encrypted bytes): the
// function passes pointers to its own aligned stack frame to three callees.
// Kept for a future checker or re-run; never passed, not verified.

/// Emit one record per live owner into `out`, advancing `*cursor` each time.
///
/// An iterator object on the stack (seeded with the constant `0x4479c000`)
/// walks the owners; each entry whose state word is clear and whose kind hook
/// `c3(obj+0x44, 0x16b)` succeeds appends four words (one integer, two floats,
/// one integer from the entry's detail block) at `out + *cursor * 16` and
/// bumps `*cursor`. Returns nothing meaningful.
///
/// Original: cdecl, two stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c82470(out: u32, cursor: u32) -> u32 {
    unsafe {
        const SEED: u32 = 0x4479c000;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        let mut seed = [SEED, 0, 0, 0, 0, 0, 0, 0];
        let mut iter = [0u32; 8];
        lf_checker_rt::callee_thiscall!(C1, u32, seed.as_mut_ptr() as u32, 1u32, 0u32, 0u32);
        let mut item: u32 = lf_checker_rt::callee_thiscall!(C2, u32, iter.as_mut_ptr() as u32);
        while item != 0 {
            let st = ((item + 0x6c) as *const u32).read_unaligned();
            let blocked = st != 0 && (((st + 0x0e) as *const u8).read() != 0);
            if !blocked {
                let obj = ((item + 0x224) as *const u32).read_unaligned().wrapping_add(0x44);
                let ok: u32 = lf_checker_rt::callee_thiscall!(C3, u32, obj, 0x16bu32);
                if ok != 0 {
                    let n = (cursor as *const u32).read_unaligned();
                    let dst = out.wrapping_add(n.wrapping_shl(4));
                    (cursor as *mut u32).write_unaligned(n.wrapping_add(1));
                    let det = ((item + 0x20) as *const u32).read_unaligned();
                    (dst as *mut u32)
                        .write_unaligned(((det + 0x30) as *const u32).read_unaligned());
                    ((dst + 4) as *mut u32)
                        .write_unaligned(((det + 0x34) as *const u32).read_unaligned());
                    ((dst + 8) as *mut u32)
                        .write_unaligned(((det + 0x38) as *const u32).read_unaligned());
                    ((dst + 12) as *mut u32)
                        .write_unaligned(((det + 0x3c) as *const u32).read_unaligned());
                }
            }
            item = lf_checker_rt::callee_thiscall!(C2, u32, iter.as_mut_ptr() as u32);
        }
        lf_checker_rt::callee_thiscall!(C4, u32, iter.as_mut_ptr() as u32)
    }
});
