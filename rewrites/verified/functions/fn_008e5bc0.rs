// original: 0x008E5BC0 refcount_release_or_recompute (proposed)

/// Release a reference, or recompute the cached sums when unreferenced.
///
/// `COUNT` holds an unsigned reference count. Above 1 it is decremented
/// and the new value returned. At exactly 1 the handle is released: unless
/// `FLAG_A` is set while `FLAG_B` is also set (which returns 1 with
/// nothing done), `FLAG_A` is cleared and the releaser (callee 1, stdcall)
/// runs on `HANDLE`; then `HANDLE` and `COUNT` are zeroed and the
/// releaser's answer returned. At 0 the cached sums are recomputed: unless
/// `MODE` is already set, the three floats `F0 + F1 + F2` (in that order)
/// are passed to the updater (callee 2, thiscall on `CB_THIS`); the two
/// floats `G0 + G1` are always stored to `G_OUT`; and unless `FLAG_B` is
/// set, `MODE` and `FLAG_A` are cleared. The updater's answer (or 0 when
/// it did not run) is returned.
///
/// Original: 0x008E5BC0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008E5BC0(obj: u32) -> u32 {
    unsafe {
        /// Unsigned reference count.
        const COUNT: u32 = 0x470;
        /// Release guard flag.
        const FLAG_A: u32 = 0x46C;
        /// Keep-alive flag.
        const FLAG_B: u32 = 0x468;
        /// Owned handle, released at count 1.
        const HANDLE: u32 = 0x474;
        /// Recompute mode byte.
        const MODE: u32 = 0x480;
        /// Updater target object.
        const CB_THIS: u32 = 0x484;
        /// Updater float operands, summed in order F0 + F1 + F2.
        const F0: u32 = 0x494;
        const F1: u32 = 0x48C;
        const F2: u32 = 0x488;
        /// Cached-sum operands and output.
        const G0: u32 = 0x498;
        const G1: u32 = 0x490;
        const G_OUT: u32 = 0x49C;
        /// Releaser callee id.
        const REL: u32 = 1;
        /// Updater callee id.
        const UPD: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let count = rd32(obj.wrapping_add(COUNT));
        if count > 1 {
            let n = count.wrapping_sub(1);
            wr32(obj.wrapping_add(COUNT), n);
            return n;
        }
        if count == 1 {
            if rd32(obj.wrapping_add(FLAG_A)) != 0 {
                if rd32(obj.wrapping_add(FLAG_B)) != 0 {
                    return 1;
                }
                wr32(obj.wrapping_add(FLAG_A), 0);
            }
            let ans: u32 =
                lf_checker_rt::callee_stdcall!(REL, u32, rd32(obj.wrapping_add(HANDLE)));
            wr32(obj.wrapping_add(HANDLE), 0);
            wr32(obj.wrapping_add(COUNT), 0);
            return ans;
        }
        let mut ans: u32 = 0;
        if rd8(obj.wrapping_add(MODE)) == 0 {
            let f = fadd(
                fadd(rdf(obj.wrapping_add(F0)), rdf(obj.wrapping_add(F1))),
                rdf(obj.wrapping_add(F2)),
            );
            ans = lf_checker_rt::callee_thiscall!(
                UPD,
                u32,
                rd32(obj.wrapping_add(CB_THIS)),
                f.to_bits()
            );
        }
        wr32(
            obj.wrapping_add(G_OUT),
            fadd(rdf(obj.wrapping_add(G0)), rdf(obj.wrapping_add(G1))).to_bits(),
        );
        if rd32(obj.wrapping_add(FLAG_B)) == 0 {
            wr8(obj.wrapping_add(MODE), 0);
            wr32(obj.wrapping_add(FLAG_A), 0);
        }
        ans
    }
});
