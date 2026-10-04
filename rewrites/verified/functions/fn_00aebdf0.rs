// original: 0x00aebdf0 timing_gate_select (proposed)

/// Gate on two objects' state, then dispatch by a table-driven mode query.
///
/// `obj` and `other` are objects with flag words at `+0x24` (and `+0x3c` on
/// `other`); `out` receives a float and `n` is a small selector. Returns 2
/// when the flag gate for the current `other` mode fails, 1 when the locked
/// row marker is set and the lock global is armed, otherwise asks the mode
/// of the row selected by the half word at `obj+0x2e` through the row table
/// global (virtual slot `+0xc` on the row's object):
/// mode 3 runs the reset callee and returns 0 if it settles, else falls into
/// the distance path; modes 5-6 run the two probe callees and, when armed,
/// the distance-report callee with the distance between the position at
/// `[esi+0x20]+0x30` (or `obj+0x10`) and `other+0x910`, returning 0;
/// any other mode stores the distance between the position selected through
/// `obj+0x4c` (or the same fallback) and `other+0x910` into `out`, clamps it
/// against the scaled limit, and tail-calls the follow-up with the result.
///
/// Cdecl, four stack words. The original spills the row object over its own
/// fourth argument slot; the rewrite keeps it in a local. Float order is
/// the original's (note the two distance sums use opposite orders).
lf_checker_rt::export!(cdecl, rw_00aebdf0(obj: u32, out: u32, n: u32, other: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x24;
        const MODE: u32 = 0x3c;
        const ROWSEL: u32 = 0x50;
        const LOCKW: u32 = 0x28;
        const ROWIDX: u32 = 0x2e;
        const POSP: u32 = 0x20;
        const POSFALLBACK: u32 = 0x10;
        const LINK: u32 = 0x4c;
        const LOCK_GLOBAL: u32 = 0x015c7f18;
        const ROW_TABLE: u32 = 0x01295cd8;
        const VT_SLOT: u32 = 0x0c;
        const EPS: u32 = 0x00fe8c10;
        const BASE: u32 = 0x0103f714;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let flags = rd32(obj + FLAGS);
        if rd32(other + MODE) == 2 {
            if flags & 0x20000 != 0 {
                return 2;
            }
        } else if flags & 0x10000 != 0 {
            return 2;
        }
        if flags & 0x4000000 != 0
            && rd32(obj + LOCKW) & 0x3c0 == 0x100
            && (lf_checker_rt::global::<u8>(LOCK_GLOBAL)).read() != 0
        {
            return 1;
        }
        let sel = (obj + ROWIDX) as *const i16;
        let row_entry = lf_checker_rt::relocated(ROW_TABLE)
            .wrapping_add((sel.read_unaligned() as i32 as u32).wrapping_mul(4));
        let row_obj = rd32(row_entry);
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(row_obj) + VT_SLOT) as usize);
        let al = (slot(row_obj) & 0xff) as u8;
        if al == 3 {
            let settled: u32 = lf_checker_rt::callee_cdecl!(5, u32, obj, 0u32, 1u32, 0u32);
            if settled & 0xff != 0 {
                return 0;
            }
        } else if al.wrapping_sub(5) <= 1 {
            if flags & 0x20 == 0 {
                return 0;
            }
            let p1: u32 = lf_checker_rt::callee_cdecl!(2, u32, obj, other, 0u32);
            if p1 & 0xff != 0 {
                return 2;
            }
            let p2: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj, other);
            if p2 & 0xff != 0 {
                return 2;
            }
            if flags & 0x1000 == 0 {
                return 1;
            }
            let mask = 1u32.wrapping_shl(n);
            let pp = rd32(obj + POSP);
            let pos = if pp != 0 { pp.wrapping_add(0x30) } else { obj + POSFALLBACK };
            let dx = sub(rdf(pos), rdf(other + 0x910));
            let dy = sub(rdf(pos + 4), rdf(other + 0x914));
            let dz = sub(rdf(pos + 8), rdf(other + 0x918));
            let d = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz)).sqrt();
            lf_checker_rt::callee_cdecl!(4, u32, obj, d.to_bits(), mask, 0u32);
            return 0;
        }
        // Distance path (modes other than 3, 5, 6, plus unsettled mode 3).
        let link = rd32(obj + LINK);
        let pos = if link != 0 {
            let w = rd32(link + POSP);
            if w != 0 { w.wrapping_add(0x30) } else { link.wrapping_add(0x10) }
        } else {
            let pp = rd32(obj + POSP);
            if pp != 0 { pp.wrapping_add(0x30) } else { obj + POSFALLBACK }
        };
        let dy = sub(rdf(pos + 4), rdf(other + 0x914));
        let dx = sub(rdf(pos), rdf(other + 0x910));
        let dz = sub(rdf(pos + 8), rdf(other + 0x918));
        let d = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
        (out as *mut f32).write_unaligned(d);
        let eps = (lf_checker_rt::global::<f32>(EPS)).read_unaligned();
        if d > eps {
            let lim = mul(rdf(other + 0x934), rdf(obj + ROWSEL));
            if lim > eps {
                let base = (lf_checker_rt::global::<f32>(BASE)).read_unaligned();
                if add(lim, base) > d {
                    (out as *mut f32).write_unaligned(add(sub(lim, eps), d));
                }
            }
        }
        let final_d = (out as *const f32).read_unaligned();
        // Second argument is the row object the original spilled over its own
        // fourth argument slot on the way in.
        lf_checker_rt::callee_cdecl!(6, u32, obj, row_obj, final_d.to_bits(), 1u32, n, other)
    }
});
