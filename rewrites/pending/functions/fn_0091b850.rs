// original: 0x0091B850 scale_vec4_dispatched
/// Scale a four-float vector by selected global factors, then dispatch.
///
/// Copies 16 bytes from the source pointer into a local vector. Four times it
/// queries the state helper; each answer selects one of a pair of global
/// integers (pairs at `0x0105C880`/`0x0105C87C` for lanes 1 and 3,
/// `0x0105C884`/`0x0105C888` for lanes 0 and 2), converts the winner with
/// `cvtdq2ps` and multiplies it into its lane with `mulss`. Finally it calls
/// one of two tail helpers with `(local, a1, &a2)` depending on whether the
/// flag byte is zero, and returns that call's answer. All float work uses SSE
/// intrinsics so every operation is the same single-rounded instruction as the
/// original.
export!(cdecl, rw_0091b850(a0: u32, a1: u32, a2: u32, flag: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_cvtepi32_ps, _mm_mul_ss, _mm_set_epi32,
                              _mm_set_ss};
        let s = a0 as *const u32;
        let mut t = [*s, *s.add(1), *s.add(2), *s.add(3)];
        let pairs: [(u32, u32, usize); 4] = [
            (0x105C880, 0x105C87C, 1),
            (0x105C880, 0x105C87C, 3),
            (0x105C884, 0x105C888, 0),
            (0x105C884, 0x105C888, 2),
        ];
        for (base, alt, lane) in pairs {
            let ans: u32 = callee_cdecl!(1, u32,);
            let v = if (ans & 0xFF) == 0 {
                *global::<u32>(base)
            } else {
                *global::<u32>(alt)
            };
            let vf = _mm_cvtepi32_ps(_mm_set_epi32(0, 0, 0, v as i32));
            let m = _mm_mul_ss(vf, _mm_set_ss(f32::from_bits(t[lane])));
            t[lane] = _mm_cvtss_f32(m).to_bits();
        }
        let mut a2slot = a2;
        if (flag & 0xFF) == 0 {
            callee_cdecl!(3, u32, t.as_mut_ptr() as u32, a1, &mut a2slot as *mut u32 as u32)
        } else {
            callee_cdecl!(2, u32, t.as_mut_ptr() as u32, a1, &mut a2slot as *mut u32 as u32)
        }
    }
});
