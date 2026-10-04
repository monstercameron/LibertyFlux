// original: 0x00905650 input_axis_resolve (proposed)

/// Resolve one input axis to -1, 0 or 1 by comparing the incoming value
/// against a threshold picked from the active device.
///
/// Returns 0 at once when the device lookup helper finds nothing. On the
/// first call (flag bit 0 clear) the flag is set and the threshold global
/// is primed from the sampler helper (cdecl, frame pointer; float at
/// answer `+8`); the sampler runs again on every call and its float lands
/// in the threshold global. The resolver helper (cdecl, one zero word)
/// yields the device record, or null.
///
/// With a record and mode 2 plus a set shoulder helper answer, the
/// threshold is the table float for the record's signed word at `+0x2e`
/// plus the record link's (`+0x20`) float at `+0x38`. Otherwise the
/// record's virtual slot `0x1b8` (thiscall, record in ecx) decides: an
/// answer of 3 subtracts the deadzone constant from the link float, any
/// other answer adds it. Without a record the threshold global keeps its
/// value. The bound starts as one constant, or another constant when mode
/// is 2 with no record.
///
/// Returns -1 when the value minus the bound exceeds the threshold,
/// else 1 when the threshold exceeds the bound plus the value, else 0.
/// All arithmetic is single-float in the original's operand order.
/// Original: 0x00905650 (cdecl, one float word).
lf_checker_rt::export!(cdecl, rw_00905650(farg: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x01193C70;
        const THRESH: u32 = 0x01193C6C;
        const MODE: u32 = 0x011D6FD4;
        const DEVTAB: u32 = 0x01295CD8;
        const DEADZONE: u32 = 0x00FE88E8;
        const BOUND_A: u32 = 0x00FE8A24;
        const BOUND_B: u32 = 0x00FE8998;
        const VT_SLOT: u32 = 0x1b8;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }

        let h = lf_checker_rt::callee_cdecl!(1, u32,);
        if h == 0 {
            return 0;
        }
        let mut frame = [0u32; 8];
        let base = core::hint::black_box(frame.as_mut_ptr() as u32);
        let flagp = lf_checker_rt::relocated(FLAG);
        let threshp = lf_checker_rt::relocated(THRESH);
        let flag = (flagp as *const u32).read_unaligned();
        if flag & 1 == 0 {
            (flagp as *mut u32).write_unaligned(flag | 1);
            let p = lf_checker_rt::callee_cdecl!(2, u32, base.wrapping_add(0x10));
            (threshp as *mut u32).write_unaligned(rdf(p.wrapping_add(8)).to_bits());
        }
        let p = lf_checker_rt::callee_cdecl!(2, u32, base.wrapping_add(0x10));
        (threshp as *mut u32).write_unaligned(rdf(p.wrapping_add(8)).to_bits());

        let esi = lf_checker_rt::callee_cdecl!(3, u32, 0u32);
        let mut x1: f32 = 0.0;
        if esi == 0 {
            x1 = rdf(threshp);
        } else {
            let mode = (lf_checker_rt::relocated(MODE) as *const u32).read_unaligned();
            let mut done = false;
            if mode == 2 {
                let a = lf_checker_rt::callee_cdecl!(4, u32,);
                if (a as u8) != 0 {
                    let i = (esi.wrapping_add(0x2e) as *const i16).read_unaligned() as i32;
                    let c =
                        (esi.wrapping_add(0x20) as *const u32).read_unaligned();
                    let tab = lf_checker_rt::relocated(DEVTAB);
                    let ent = (tab.wrapping_add((i as u32).wrapping_mul(4))
                        as *const u32)
                        .read_unaligned();
                    let v = add(rdf(ent.wrapping_add(0x28)), rdf(c.wrapping_add(0x38)));
                    (threshp as *mut u32).write_unaligned(v.to_bits());
                    x1 = v;
                    done = true;
                }
            }
            if !done {
                let vt = (esi as *const u32).read_unaligned();
                let slot = (vt.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let r = f(esi);
                let c = (esi.wrapping_add(0x20) as *const u32).read_unaligned();
                let v0 = rdf(c.wrapping_add(0x38));
                let k = rdf(lf_checker_rt::relocated(DEADZONE));
                let v = if r == 3 { sub(v0, k) } else { add(v0, k) };
                (threshp as *mut u32).write_unaligned(v.to_bits());
                x1 = v;
            }
        }

        let mode = (lf_checker_rt::relocated(MODE) as *const u32).read_unaligned();
        let mut x2 = rdf(lf_checker_rt::relocated(BOUND_A));
        let mut x4 = x2;
        if mode == 2 && esi == 0 {
            x2 = rdf(lf_checker_rt::relocated(BOUND_B));
            x4 = x2;
        }
        let x3 = f32::from_bits(farg);
        let x0 = sub(x3, x4);
        if x0 > x1 {
            return 0xFFFFFFFF;
        }
        x2 = add(x2, x3);
        if x1 > x2 {
            1
        } else {
            0
        }
    }
});
