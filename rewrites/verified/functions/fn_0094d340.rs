// original: 0x0094D340 NativeImpl_IS_CAR_STOPPED (symbols)

/// Report whether a velocity vector's squared length is under a threshold.
///
/// Calls the function at slot 0xEC of `obj`'s table (callee 1, thiscall
/// with `obj` in ECX and a stack-frame pointer on the stack; the contract
/// plants the stub address in the slot, skips the frame pointer and
/// snapshots the 3 pointed-to words instead). The answer points to three
/// floats whose sum of squares is computed in the original's exact order —
/// (a*a + b*b) + c*c, every operation pinned — and compared against the
/// threshold word `THRESH`: the low return byte is 1 when the threshold is
/// above or equal (comiss + setae, so NaN on either side yields 0, exactly
/// like `>=`), 0 otherwise. Only `al` is defined.
///
/// Original: 0x0094D340 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_0094D340(obj: u32) -> u32 {
    unsafe {
        const THRESH: u32 = 0xFE868C;
        const SLOT: u32 = 1;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let frame = [0u32, 0u32, 0u32];
        let v = lf_checker_rt::callee_thiscall!(SLOT, u32, obj, frame.as_ptr() as u32);
        let a = f32::from_bits((v as *const u32).read_unaligned());
        let b = f32::from_bits((v.wrapping_add(4) as *const u32).read_unaligned());
        let c = f32::from_bits((v.wrapping_add(8) as *const u32).read_unaligned());
        let sumsq = add(add(mul(a, a), mul(b, b)), mul(c, c));
        let t = f32::from_bits(
            (lf_checker_rt::global::<u32>(THRESH) as *const u32).read(),
        );
        (t >= sumsq) as u32
    }
});
