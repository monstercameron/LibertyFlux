// original: 0x00DA9A70 ped_task_probe_slots (proposed)

/// Probe up to 16 candidate slots with a two-phase search, then copy the
/// winning slot into a destination object.
///
/// Arguments (cdecl, nine stack words): `obj` (object whose word at `+0x38`
/// is forwarded to every probe call), `src_a` (first point source, or the
/// fallback selector), `src_b` (second point source), `dst` (destination
/// object for the final copy), `radius` (float bits, radial-search radius),
/// `mode` (low byte nonzero selects the two-point probe, zero the radial
/// probe), `count2` (slot budget forwarded to the second radial call),
/// `param` (opaque word forwarded to every probe call), `sel` (selector
/// override; when 0 `src_a` is used instead).
///
/// Behaviour: builds 16 records of 0x60 bytes on the stack. Every record
/// holds three copies of a 3-float triple read from one global triple, zero
/// everywhere else except a 0xFFFF marker word at record offset 0x4C.
/// Then, when the mode byte is nonzero, copies two 3-float points (from
/// `src_a` and `src_b`) into a 6-float buffer and makes the two-point probe
/// call (callee 0); otherwise makes the first radial call (callee 1) and, if
/// it reports fewer than 16 slots used, a second radial call (callee 2)
/// starting at the record it stopped at. The second radial call's second
/// argument re-reads the spilled `src_b` slot, so it is `src_b` again.
/// Then makes the select call (callee 3) over the 16 records; a -1 answer
/// returns 0, otherwise the indexed record is copied into `dst` (callee 4)
/// and 1 is returned. Both exits run the CRT security-cookie check
/// (callee 5, callee-preserved registers) with the cookie value.
///
/// Only the low byte of the return is meaningful (0 or 1); the float moves
/// are bit copies, so all bit patterns round-trip unchanged.
///
/// Original: 0x00DA9A70 (cdecl, nine stack words).
lf_checker_rt::export!(cdecl, rw_00DA9A70(obj: u32, src_a: u32, src_b: u32, dst: u32, radius: u32, mode: u32, count2: u32, param: u32, sel: u32) -> u32 {
    unsafe {
        const NREC: usize = 16;
        const REC_WORDS: usize = 24; // 0x60 bytes
        const REC_BYTES: u32 = 0x60;
        const OBJ_FIELD: u32 = 0x38;
        const MARKER: u32 = 0xFFFF;
        const INIT_TRIPLE: u32 = 0x1B4B320;
        const PROBE_OBJ: u32 = 0x12B9C78;
        const COOKIE: u32 = 0x1057FB4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let g0 = rd32(lf_checker_rt::relocated(INIT_TRIPLE));
        let g1 = rd32(lf_checker_rt::relocated(INIT_TRIPLE + 4));
        let g2 = rd32(lf_checker_rt::relocated(INIT_TRIPLE + 8));
        let probe_obj = rd32(lf_checker_rt::relocated(PROBE_OBJ));
        let field = rd32(obj.wrapping_add(OBJ_FIELD));

        let mut recs = [[0u32; REC_WORDS]; NREC];
        for r in recs.iter_mut() {
            r[4] = g0; r[5] = g1; r[6] = g2;
            r[8] = g0; r[9] = g1; r[10] = g2;
            r[12] = g0; r[13] = g1; r[14] = g2;
            r[19] = MARKER; // record offset 0x4C
        }
        let base = recs.as_mut_ptr() as u32;

        if (mode as u8) != 0 {
            let mut pts = [
                rd32(src_a), rd32(src_a.wrapping_add(4)), rd32(src_a.wrapping_add(8)),
                0,
                rd32(src_b), rd32(src_b.wrapping_add(4)), rd32(src_b.wrapping_add(8)),
            ];
            let _r: u32 = lf_checker_rt::callee_thiscall!(0, u32, probe_obj,
                pts.as_mut_ptr() as u32, base, field, param, 0xFFFF_FFFF, 7, 0x10, 0);
        } else {
            let used: u32 = lf_checker_rt::callee_thiscall!(1, u32, probe_obj,
                src_a, radius, base, field, param, 0xFFFF_FFFF, 7, 0x10, 0);
            let rem = 0x10u32.wrapping_sub(used);
            if (rem as i32) > 0 {
                let rec = base.wrapping_add(used.wrapping_mul(REC_BYTES));
                // Second argument re-reads the spilled src_b slot.
                let _r: u32 = lf_checker_rt::callee_thiscall!(2, u32, probe_obj,
                    src_a, src_b, radius, rec, field, param, 0xFFFF_FFFF, 7, count2, rem, 0);
            }
        }

        let which = if sel != 0 { sel } else { src_a };
        let idx: u32 = lf_checker_rt::callee_cdecl!(3, u32, obj, base, 0x10, which, 0, 0, 0);
        let cookie = rd32(lf_checker_rt::relocated(COOKIE));
        if idx == 0xFFFF_FFFF {
            let _c: u32 = lf_checker_rt::callee_thiscall!(5, u32, cookie);
            return 0;
        }
        let rec = base.wrapping_add(idx.wrapping_mul(REC_BYTES));
        let _r: u32 = lf_checker_rt::callee_thiscall!(4, u32, dst, rec);
        let _c: u32 = lf_checker_rt::callee_thiscall!(5, u32, cookie);
        1
    }
});
