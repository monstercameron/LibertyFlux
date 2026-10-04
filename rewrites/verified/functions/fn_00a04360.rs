// original: 0x00A04360 frag_traced_dispatch (proposed)

/// Build a trace record from four floats and dispatch on the trace test.
///
/// Forms `t = a3 * a3 * k` with the global factor (original operand order),
/// packs `t` and the inputs into two frame blocks, and runs the trace
/// tester callee with (`block_b`, const, `block_a`, 0x10, 0xd), where
/// `block_b` is [`a0`, `a1`, `a2`] and `block_a` starts [`t`, out]. The
/// tester reports through `block_a[1]` (an out-slot the original reads
/// back, ignoring the register answer), so the stub writes it. Later words
/// of both blocks are uninitialised holes, so only the first three of
/// `block_b` and the first two of `block_a` are compared. When the slot
/// reads zero, the out-pointer `a4` receives zero and `a4` is returned;
/// otherwise the resolver callee runs on a global object with the slot
/// value, its answer is stored through `a4` and returned.
///
/// Original: 0x00A04360 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00A04360(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const FACTOR: u32 = 0x00FE8AB8;
        const TARGET: u32 = 0x0094B7D0;
        const RESOLVER_OBJ: u32 = 0x01632C60;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let k = (lf_checker_rt::relocated(FACTOR) as *const f32).read_unaligned();
        let f3 = f32::from_bits(a3);
        let t = mul(mul(f3, f3), k);
        let block_b = [a0, a1, a2, 0u32];
        let mut block_a = [t.to_bits(), 0u32, 0, 0];
        lf_checker_rt::callee_cdecl!(1, u32, block_b.as_ptr() as u32,
            lf_checker_rt::relocated(TARGET), block_a.as_ptr() as u32, 0x10u32, 0x0Du32);
        // Volatile: the stub wrote the out-slot through the passed pointer.
        let r = core::ptr::read_volatile(&block_a[1]);
        if r == 0 {
            (a4 as *mut u32).write_unaligned(0);
            return a4;
        }
        let obj = (lf_checker_rt::relocated(RESOLVER_OBJ) as *const u32).read_unaligned();
        let ans = lf_checker_rt::callee_thiscall!(2, u32, obj, r);
        (a4 as *mut u32).write_unaligned(ans);
        ans
    }
});
