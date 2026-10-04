// original: 0x00b03b40 threshold_dispatch (proposed)

/// Measure a value through the object, compare it to a threshold, and
/// dispatch to one of four back ends chosen by two flags and a mode bit.
///
/// Returns 0 at once when the object is disabled (bit `DISABLE_BIT` of the
/// dword at `+0x24`) or when both selector slots of the state (`+0x8F8` and
/// `+0x8FC`, defaulting to the global state when the argument is null) hold
/// negative values. Otherwise the measure hook (virtual slot `+0x58`) yields
/// a float `f`, and `mode` is bits 10 and up of the dword at `+0x8E8` with
/// bits 1-7 cleared (only its low byte is ever tested).
/// When the threshold strictly exceeds `f` and the enable flag is set, the
/// fill hook (virtual slot `+0x50`) fills a two-word scratch buffer, then the
/// select flag picks the back end: select set with mode clear calls back end
/// A (five words: table slot for `+0x8F8`, buffer, `f`, 0, 0) and returns its
/// byte; select clear with mode set, or select set with mode set, calls back
/// end B (three words: table slot for `+0x8FC`, buffer, `f`) and returns the
/// inverted byte; the remaining combination returns 0.
/// Otherwise (threshold at or below `f`, unordered, or enable flag clear) a
/// 32-byte entry picked by the halfword at `+0x2E` from the entry table is
/// split into two 16-byte halves, the scan call takes the end of the first
/// half, and the select flag picks again: select set with mode clear calls
/// back end C (six words: table slot for `+0x8F8`, first half, second half,
/// end, 0, 0); any mode-set combination calls back end D (five words: shared
/// `this`, table slot for `+0x8FC`, first half, second half, end) and returns
/// the inverted byte; a negative table index returns 0 instead of calling.
/// The original pushes its words left to right, so the last pushed is the
/// first argument; a dead register word it pushes is overwritten with `f`
/// before each of the first two back-end calls, and only `f` is passed on.
/// The words past the first half are unread stack (the checker's defined
/// fill), materialised as zeros.
///
/// Original: 0x00b03b40 (thiscall, this in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00b03b40(this: u32, arg: u32) -> u32 {
    unsafe {
        const DISABLE_BIT: u32 = 0x0800_0000;
        const FLAG_OFF: u32 = 0x24;
        const MODE_OFF: u32 = 0x8e8;
        const SEL_A_OFF: u32 = 0x8f8;
        const SEL_B_OFF: u32 = 0x8fc;
        const IDX_OFF: u32 = 0x2e;
        const MODE_MASK: u32 = 0xffff_ff01;
        const VT_FILL: u32 = 0x50;
        const VT_MEASURE: u32 = 0x58;
        const M_SCAN: u32 = 3;
        const M_A: u32 = 4;
        const M_B: u32 = 5;
        const M_C: u32 = 6;
        const M_D: u32 = 7;
        const G_STATE: u32 = 0x012f_b1b8;
        const G_THRESH: u32 = 0x0104_0064;
        const G_ENABLE: u32 = 0x0104_0060;
        const G_SELECT: u32 = 0x0103_f6e4;
        const G_THIS: u32 = 0x0160_1098;
        const G_TABLE: u32 = 0x0160_10ac;
        const G_ENTRIES: u32 = 0x0129_5cd8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        let backend_b = |g_this: u32, f: f32, bufp: u32, esi: u32| -> u32 {
            unsafe {
                let idx = rd32(esi.wrapping_add(SEL_B_OFF)) as i32;
                if idx <= -1 {
                    return 0;
                }
                let slot = g32(G_TABLE.wrapping_add(idx as u32 * 4));
                let r: u32 = lf_checker_rt::callee_thiscall!(M_B, u32, g_this, slot, bufp, f.to_bits(),);
                u32::from((r & 0xff) == 0)
            }
        };
        let backend_d = |g_this: u32, endp: u32, second: u32, first: u32, esi: u32| -> u32 {
            unsafe {
                let idx = rd32(esi.wrapping_add(SEL_B_OFF)) as i32;
                if idx <= -1 {
                    return 0;
                }
                let slot = g32(G_TABLE.wrapping_add(idx as u32 * 4));
                let r: u32 = lf_checker_rt::callee_thiscall!(M_D, u32, g_this, g_this, slot, first, second, endp,);
                u32::from((r & 0xff) == 0)
            }
        };

        if rd32(this.wrapping_add(FLAG_OFF)) & DISABLE_BIT != 0 {
            return 0;
        }
        let shared = g32(G_STATE);
        let esi = if arg != 0 { arg } else { shared };
        if (rd32(esi.wrapping_add(SEL_A_OFF)) as i32) < 0
            && (rd32(esi.wrapping_add(SEL_B_OFF)) as i32) < 0
        {
            return 0;
        }
        let vt = rd32(this);
        let measure: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_MEASURE)) as usize);
        let f = measure(this);
        let mode = (rd32(esi.wrapping_add(MODE_OFF)) >> 10) & MODE_MASK;
        let fresh = rdf(lf_checker_rt::relocated(G_THRESH)) > f;
        if fresh && rd8(lf_checker_rt::relocated(G_ENABLE)) != 0 {
            let mut buf = [0u32; 2];
            let fill: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_FILL)) as usize);
            fill(this, buf.as_mut_ptr() as u32);
            let g_this = g32(G_THIS);
            let bufp = buf.as_mut_ptr() as u32;
            if rd8(lf_checker_rt::relocated(G_SELECT)) != 0 {
                if mode as u8 != 0 {
                    return backend_b(g_this, f, bufp, esi);
                }
                let idx = rd32(esi.wrapping_add(SEL_A_OFF)) as i32;
                if idx <= -1 {
                    return 0;
                }
                let slot = g32(G_TABLE.wrapping_add(idx as u32 * 4));
                let r: u32 = lf_checker_rt::callee_thiscall!(M_A, u32, g_this, slot, bufp, f.to_bits(), 0u32, 0u32,);
                return r;
            }
            if mode as u8 == 0 {
                return 0;
            }
            return backend_b(g_this, f, bufp, esi);
        }
        // Threshold path: split the picked entry and scan it.
        let idxw = rd16(this.wrapping_add(IDX_OFF)) as u16 as i32;
        let entry = g32(G_ENTRIES.wrapping_add((idxw as u32).wrapping_mul(4)));
        let mut first = [0u32; 8];
        let mut second = [0u32; 4];
        for i in 0..4 {
            first[i] = rd32(entry.wrapping_add(0x20 + i as u32 * 4));
            second[i] = rd32(entry.wrapping_add(0x30 + i as u32 * 4));
        }
        let endp = first.as_mut_ptr().wrapping_add(4) as u32;
        let _: u32 = lf_checker_rt::callee_stdcall!(M_SCAN, u32, endp,);
        let g_this = g32(G_THIS);
        if rd8(lf_checker_rt::relocated(G_SELECT)) == 0 {
            if mode as u8 == 0 {
                return 0;
            }
            return backend_d(g_this, endp, second.as_mut_ptr() as u32, first.as_mut_ptr() as u32, esi);
        }
        if mode as u8 != 0 {
            return backend_d(g_this, endp, second.as_mut_ptr() as u32, first.as_mut_ptr() as u32, esi);
        }
        let idx = rd32(esi.wrapping_add(SEL_A_OFF)) as i32;
        if idx <= -1 {
            return 0;
        }
        let slot = g32(G_TABLE.wrapping_add(idx as u32 * 4));
        let r: u32 = lf_checker_rt::callee_thiscall!(
            M_C, u32, g_this, slot, first.as_mut_ptr() as u32, second.as_mut_ptr() as u32, endp, 0u32, 0u32,
        );
        r
    }
});
