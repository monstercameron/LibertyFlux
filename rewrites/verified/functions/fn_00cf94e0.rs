// original: 0x00CF94E0 task_result_scan (proposed)
//
// Scans a task list for ladder-climb result entries (kind 0x0e) and keeps the
// best match in two caller structs.
//
// Arguments (cdecl, six stack words): `obj` (owner object; u16 task id at
// +0x2e), `skip` (an entry index to skip, or -1 to take the filter path and
// write the match back), `best_a`/`best_b` (four-float best-so-far structs:
// x at +0, z at +4, height at +8, spare at +0xc), `out_vec` (four-float
// direction vector, refreshed on a match when `skip` is -1), `out_flag`
// (byte slot receiving the callee's flag on an improved match).
//
// Behaviour: the task id selects a table entry from the global task table;
// callee 1 (thiscall/0) answers the entry count, callee 2 (thiscall/1)
// answers the entry object for an index, and the entry's vtable slot 1
// (planted callee 3) answers its kind. For kind 0x0e, callee 4 (cdecl/6)
// fills four out areas (three float quads and one flag byte): the second
// quad holds the candidate x/z at +0/+4 and the new height and spare at
// +8/+0xc, the third quad the direction triple at +0/+4/+8 and a spare at
// +0xc. Note the original reads x/z and the direction BEFORE its `(an instruction of the original)`
// cleanup, i.e. at pushed-frame offsets, which is why they sit at the
// quad starts. Unless `skip` is -1 the candidate must pass three gates:
// |x - best_a.x| and |z - best_a.z| both under 0.3, and the dot product of
// `out_vec` with the answered direction above 0.9. A passing candidate with
// a new height above best_a's height refreshes best_a and the flag; best_b
// takes the whole first quad when its own height is above that quad's third
// word. Returns 1 in AL when any entry matched, else 0.
//
// The skip register is reloaded from the incoming `skip` slot after every
// non-skipped entry, and a match on the -1 path writes the matched index
// back into that slot (the worker does not compare above-frame stack on the
// rewrite side, so the contract runs with the stack check off; the slot is
// modelled locally and its reloads affect later iterations exactly as in the
// original). All float comparisons are ordered (`>`), matching comiss+jbe
// including NaN (unordered never passes). Float operation order is the
// original's, pinned with black_box.
lf_checker_rt::export!(cdecl, rw_00CF94E0(obj: u32, skip: u32, best_a: u32, best_b: u32, out_vec: u32, out_flag: u32) -> u32 {
    unsafe {
        const TASK_ID_OFF: u32 = 0x2e;
        const TASK_TABLE: u32 = 0x01295CD8;
        const EPS_ADDR: u32 = 0x00FE87E8;
        const DOT_MIN_ADDR: u32 = 0x00FE88BC;
        const ABS_MASK_ADDR: u32 = 0x00FE8F80;
        const WANT_KIND: u8 = 0x0e;
        const NO_SKIP: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let eps_bits: u32 = (lf_checker_rt::global::<u32>(EPS_ADDR) as *const u32).read_unaligned();
        let eps = f32::from_bits(eps_bits);
        let dot_min = f32::from_bits((lf_checker_rt::global::<u32>(DOT_MIN_ADDR) as *const u32).read_unaligned());
        let abs_mask = (lf_checker_rt::global::<u32>(ABS_MASK_ADDR) as *const u32).read_unaligned();

        let idx = ((obj + TASK_ID_OFF) as *const u16).read_unaligned() as i16 as i32;
        let entry = ((lf_checker_rt::relocated(TASK_TABLE).wrapping_add((idx * 4) as u32)) as *const u32)
            .read_unaligned();
        let count = lf_checker_rt::callee_thiscall!(1, u32, entry) as i32;
        let mut found: u8 = 0;
        let mut slot = skip;
        if count > 0 {
            let mut skip_reg = skip;
            let mut esi: u32 = 0;
            while (esi as i32) < count {
                if esi == skip_reg {
                    esi += 1;
                    continue;
                }
                let subtask = lf_checker_rt::callee_thiscall!(2, u32, entry, esi);
                let vt = rd32(subtask);
                let kind_fn: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + 4) as usize);
                let kind = (kind_fn(subtask) & 0xFF) as u8;
                if kind != WANT_KIND {
                    skip_reg = slot;
                    esi += 1;
                    continue;
                }
                let mut o2 = [0u32; 4];
                let mut o3 = [0u32; 4];
                let mut o4 = [0u32; 4];
                let mut o5 = 0u32;
                lf_checker_rt::callee_cdecl!(
                    4, u32, obj, esi,
                    o2.as_mut_ptr() as u32,
                    o3.as_mut_ptr() as u32,
                    o4.as_mut_ptr() as u32,
                    &mut o5 as *mut u32 as u32
                );
                skip_reg = slot;
                let x = f32::from_bits(o3[0]);
                let z = f32::from_bits(o3[1]);
                let d0 = f32::from_bits(o4[0]);
                let d1 = f32::from_bits(o4[1]);
                let d2 = f32::from_bits(o4[2]);
                if skip_reg != NO_SKIP {
                    let dx = f32::from_bits(sub(x, rdf(best_a)).to_bits() & abs_mask);
                    if !(eps > dx) {
                        esi += 1;
                        continue;
                    }
                    let dz = f32::from_bits(sub(z, rdf(best_a + 4)).to_bits() & abs_mask);
                    if !(eps > dz) {
                        esi += 1;
                        continue;
                    }
                    let dot = add(
                        add(mul(rdf(out_vec + 4), d1), mul(rdf(out_vec), d0)),
                        mul(rdf(out_vec + 8), d2),
                    );
                    if !(dot > dot_min) {
                        esi += 1;
                        continue;
                    }
                }
                let new_h = f32::from_bits(o3[2]);
                if new_h > rdf(best_a + 8) {
                    wrf(best_a + 8, new_h);
                    wrf(best_a, x);
                    wrf(best_a + 4, z);
                    wrf(best_a + 0xc, f32::from_bits(o3[3]));
                    (out_flag as *mut u8).write((o5 & 0xFF) as u8);
                }
                // Note: best_b compares against and stores the FIRST quad
                // (o2), read late at plain-frame offsets, not the x/z above.
                if rdf(best_b + 8) > f32::from_bits(o2[2]) {
                    wrf(best_b, f32::from_bits(o2[0]));
                    wrf(best_b + 4, f32::from_bits(o2[1]));
                    wrf(best_b + 8, f32::from_bits(o2[2]));
                    wrf(best_b + 0xc, f32::from_bits(o2[3]));
                }
                found = 1;
                if skip_reg == NO_SKIP {
                    wrf(out_vec, d0);
                    wrf(out_vec + 4, d1);
                    wrf(out_vec + 8, d2);
                    wrf(out_vec + 0xc, f32::from_bits(o4[3]));
                    skip_reg = esi;
                    slot = esi;
                }
                esi += 1;
            }
        }
        found as u32
    }
});
