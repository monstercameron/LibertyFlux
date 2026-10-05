// original: 0x00a1ef80 ped_task_aim_blend (proposed)

/// Blend a ped task's aim offsets toward a target direction.
///
/// `this` points to the task record, `a1` to a target object with a virtual
/// table, `a2` to a fallback record. Two gates run first: the gate callee
/// (id 1, thiscall on `this`, one stack argument) is tried with `a1`, and
/// only when its low byte comes back zero is it retried with `a2`. When both
/// return a zero low byte the function only sets bit 1 of the flag byte at
/// `this+0x38c` and returns the second answer.
///
/// Otherwise the hook at virtual slot `+0xec` of `a1`'s table is called
/// twice (id 2, thiscall on `a1`, one stack argument pointing at a scratch
/// slot whose contents are never read back). The first call answers a
/// pointer to three floats; their length `s` gives a factor: 0 when `s` is
/// zero, else `s / s` (1, or NaN when `s` is NaN or infinite). The second
/// call answers another three floats, scaled by the factor and then by the
/// global float, and accumulated into six cells at `this+0x230..0x24c`.
///
/// The accumulation order is the original's: cell 0 adds (scaled, cell),
/// cells 1, 2, 4 and 5 add (cell, scaled), cell 3 adds (scaled, cell). All
/// float operation orders below are the original's.
///
/// Original: 0x00a1ef80 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a1ef80(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0xec;
        const CELLS: u32 = 0x230;
        const FLAG_BYTE: u32 = 0x38c;
        const GLOBAL_SCALE: u32 = 0x11735cc;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        // Two gates: a1 first, a2 only when the first low byte is zero.
        let first: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, a1);
        if (first as u8) == 0 {
            let second: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, a2);
            if (second as u8) == 0 {
                wr8(this + FLAG_BYTE, rd8(this + FLAG_BYTE) | 2);
                return second;
            }
        }

        // Hook from a1's virtual table, called twice with a scratch slot.
        let slot: u32 = rd32(rd32(a1) + VTABLE_SLOT);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let mut scratch = [0u32; 4];
        let triple = hook(a1, scratch.as_mut_ptr() as u32);
        // Length of the first triple: x*x + y*y + z*z in that order.
        let x = rdf(triple);
        let y = rdf(triple + 4);
        let z = rdf(triple + 8);
        let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
        let len = len2.sqrt();
        // The original tests the length through the compare flags: zero
        // (either sign) yields a zero factor, anything else length/length.
        let factor = if len == 0.0 { 0.0 } else { div(len, len) };

        let scale: f32 = f32::from_bits(rd32(lf_checker_rt::relocated(GLOBAL_SCALE)));
        let vec = hook(a1, scratch.as_mut_ptr() as u32);
        // Scale by the factor, then by the global: (v*f) first, then the
        // global times the x lane and the y/z lanes times the global.
        let bx = mul(rdf(vec), factor);
        let by = mul(rdf(vec + 4), factor);
        let bz = mul(rdf(vec + 8), factor);
        let cx = mul(scale, bx);
        let cy = mul(by, scale);
        let cz = mul(bz, scale);
        wrf(this + CELLS, add(cx, rdf(this + CELLS)));
        wrf(this + CELLS + 4, add(rdf(this + CELLS + 4), cy));
        wrf(this + CELLS + 8, add(rdf(this + CELLS + 8), cz));
        wrf(this + CELLS + 0x10, add(cx, rdf(this + CELLS + 0x10)));
        wrf(this + CELLS + 0x14, add(rdf(this + CELLS + 0x14), cy));
        wrf(this + CELLS + 0x18, add(rdf(this + CELLS + 0x18), cz));
        wr8(this + FLAG_BYTE, rd8(this + FLAG_BYTE) | 2);
        vec
    }
});
