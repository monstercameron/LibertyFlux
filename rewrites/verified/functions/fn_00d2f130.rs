// original: 0x00d2f130 task_route_point_interpolate (proposed)

/// Interpolate a point along a route entry, or fetch the default entry.
///
/// `this` points to the task object: dword at `+0x70` is the route table
/// (16-byte entries), the byte at `+0x98` selects the entry, and the dword at
/// `+0xd8` holds state flags. `obj` is an auxiliary object whose dword at
/// `+0x20` (plus `0x30`) is passed to the solver callee; `out` receives 16
/// bytes (three interpolated floats and one word from the callee).
///
/// Nothing is read unless flag `0x200` is set, else the result is 0. When the
/// selector byte is zero, the 16 bytes at `table + 0x10` are copied to `out`
/// and the result is 1. Otherwise the entry at `table + (byte as i8) * 16`
/// supplies two endpoints (`[0..12)` and `[0x10..0x1c)`); the per-lane delta
/// `hi - lo` is passed with the entry pointer and the `obj`-derived word to
/// the solver, which returns a factor `t` and fills a fourth word. `out` gets
/// `lo + delta * t` per lane (operations in the original's order) plus the
/// callee's fourth word, and the result is 1.
///
/// Original: 0x00d2f130 (thiscall: object in ECX, two stack words, callee
/// pops 8, boolean result in AL; the solver is cdecl with three stack words
/// returning f32 in ST0).
lf_checker_rt::export!(thiscall, rw_00d2f130(this: u32, obj: u32, out: u32) -> u8 {
    unsafe {
        const TABLE_PTR: u32 = 0x70;
        const SELECTOR: u32 = 0x98;
        const STATE_FLAGS: u32 = 0xd8;
        const FLAG_READY: u32 = 0x200;
        const ENTRY_BYTES: i32 = 16;
        const OBJ_WORD: u32 = 0x20;
        const SOLVER_BIAS: u32 = 0x30;
        const SOLVER: u32 = 0;

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
            unsafe { ((a) as *mut u32).write_unaligned(v.to_bits()) }
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

        let flags = rd32(this + STATE_FLAGS);
        if flags & FLAG_READY == 0 {
            return 0;
        }
        let table = rd32(this + TABLE_PTR);
        let sel = ((this + SELECTOR) as *const u8).read();
        if sel == 0 {
            let src = table.wrapping_add(0x10);
            for word in 0..4u32 {
                ((out + word * 4) as *mut u32).write_unaligned(rd32(src + word * 4));
            }
            return 1;
        }
        let entry = table.wrapping_add((sel as i8 as i32).wrapping_mul(ENTRY_BYTES) as u32);
        let lo0 = rdf(entry);
        let lo1 = rdf(entry + 4);
        let lo2 = rdf(entry + 8);
        let d0 = sub(rdf(entry + 0x10), lo0);
        let d1 = sub(rdf(entry + 0x14), lo1);
        let d2 = sub(rdf(entry + 0x18), lo2);
        let arg = rd32(obj + OBJ_WORD).wrapping_add(SOLVER_BIAS);
        let mut deltas = [d0.to_bits(), d1.to_bits(), d2.to_bits(), 0u32];
        let t: f32 = lf_checker_rt::callee_cdecl!(SOLVER, f32, entry, deltas.as_mut_ptr() as u32, arg);
        wrf(out, add(mul(d0, t), lo0));
        wrf(out + 4, add(mul(d1, t), lo1));
        wrf(out + 12, f32::from_bits(deltas[3]));
        wrf(out + 8, add(mul(d2, t), lo2));
        1
    }
});
