// original: 0x00B022E0 allocate_indexed_handles (proposed)

/// Hands out sequential handle numbers and registers the new object.
///
/// `out1`, `out2` and `out3` each receive -1 first; then `out1` receives the
/// current value of counter A (a global) and the counter is incremented. When
/// the low byte of `flags` is zero the function returns there. Otherwise the
/// object `obj` is asked through its virtual slot at +0x38 (thiscall, no
/// arguments, only AL read): a zero answer continues below, a non-zero
/// answer continues only when the word at `obj+0x3c` equals 1 and returns
/// otherwise.
///
/// The rest registers the object under a second counter: `base` is read from
/// a global and adjusted by +0xb0, and when bit 0x400 of the word at
/// `obj+0x8e8` is set, counter B (another global) is dealt into `out3`,
/// else into `out2`, and incremented. The dealt value indexes a global
/// pointer table; three helpers are invoked with the owning context (a
/// global) in ECX: the opener with the table entry, the filler with five
/// words (three pointers derived from `base+0x40`, the `out3` pointer and
/// the table entry), and the closer with either 0 (first case) or `obj`
/// (second case) plus the table entry. In the second case a fourth helper
/// (cdecl, callee keeps the stack) also sees the table entry. Original
/// convention: cdecl, five stack words, caller cleans, no return value.
lf_checker_rt::export!(cdecl, rw_00B022E0(out1: u32, out2: u32, out3: u32, obj: u32, flags: u32) -> u32 {
    unsafe {
        const NONE: u32 = 0xFFFFFFFF;
        const CTR_A: u32 = 0x016010A4;
        const CTR_B: u32 = 0x016010A8;
        const CTX: u32 = 0x01601098;
        const TABLE: u32 = 0x016010AC;
        const BASE: u32 = 0x012FB1B8;
        const VT_SLOT: u32 = 0x38;
        const MODE_OFF: u32 = 0x3c;
        const KIND_OFF: u32 = 0x8e8;
        const KIND_BIT: u32 = 0x400;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }

        wr32(out3, NONE);
        wr32(out2, NONE);
        wr32(out1, NONE);
        let ca = rd32(lf_checker_rt::relocated(CTR_A));
        wr32(out1, ca);
        wr32(lf_checker_rt::relocated(CTR_A), ca.wrapping_add(1));
        if (flags as u8) == 0 {
            return 0;
        }
        // Same load-and-call through the object as the original; both sides
        // land on the planted stub.
        let vt = rd32(obj);
        let target = rd32(vt.wrapping_add(VT_SLOT));
        let probe: extern "thiscall" fn(u32) -> u8 =
            unsafe { core::mem::transmute(target as usize) };
        if probe(obj) != 0 && rd32(obj.wrapping_add(MODE_OFF)) != 1 {
            return 0;
        }
        let base = rd32(lf_checker_rt::relocated(BASE)).wrapping_add(0xb0);
        let bx = base.wrapping_add(0x40);
        let tbase = lf_checker_rt::relocated(TABLE);
        if rd32(obj.wrapping_add(KIND_OFF)) & KIND_BIT != 0 {
            let cb = rd32(lf_checker_rt::relocated(CTR_B));
            wr32(out3, cb);
            wr32(lf_checker_rt::relocated(CTR_B), cb.wrapping_add(1));
            let idx = rd32(out3);
            let t0 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
            let cx0 = rd32(lf_checker_rt::relocated(CTX));
            lf_checker_rt::callee_thiscall!(2, u32, cx0, t0);
            let cx1 = rd32(lf_checker_rt::relocated(CTX));
            let idx = rd32(out3);
            let t1 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
            lf_checker_rt::callee_thiscall!(
                3,
                u32,
                cx1,
                bx,
                bx.wrapping_add(0x20),
                bx.wrapping_add(0x30),
                out3,
                t1
            );
            let idx = rd32(out3);
            let cx2 = rd32(lf_checker_rt::relocated(CTX));
            let t2 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
            lf_checker_rt::callee_thiscall!(4, u32, cx2, 0, t2);
        } else {
            let cb = rd32(lf_checker_rt::relocated(CTR_B));
            wr32(out2, cb);
            wr32(lf_checker_rt::relocated(CTR_B), cb.wrapping_add(1));
            let idx = rd32(out2);
            let t0 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
            let cx0 = rd32(lf_checker_rt::relocated(CTX));
            lf_checker_rt::callee_thiscall!(2, u32, cx0, t0);
            let cx1 = rd32(lf_checker_rt::relocated(CTX));
            let idx = rd32(out2);
            let t1 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
            lf_checker_rt::callee_thiscall!(
                3,
                u32,
                cx1,
                bx,
                bx.wrapping_add(0x20),
                bx.wrapping_add(0x30),
                out3,
                t1
            );
            let idx = rd32(out2);
            let cx2 = rd32(lf_checker_rt::relocated(CTX));
            let t2 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
            lf_checker_rt::callee_thiscall!(4, u32, cx2, obj, t2);
            let idx = rd32(out2);
            let t3 = rd32(tbase.wrapping_add(idx.wrapping_mul(4)));
            lf_checker_rt::callee_cdecl!(5, u32, t3);
        }
        0
    }
});
