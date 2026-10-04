// original: 0x009FD320 frag_schedule_side_a (proposed)

/// Stash the time delta for side A and schedule its callback.
///
/// Subtracts the side-A reference time global from the master time global
/// (original operand order: master minus reference), stores the difference
/// in the side-A delta global, and invokes the scheduler callee on the
/// side-A object with (`arg`, callback), where the callback is the shared
/// side-A handler. Returns the scheduler's answer.
///
/// Original: 0x009FD320 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FD320(arg: u32) -> u32 {
    unsafe {
        const MASTER: u32 = 0x00FE8B08;
        const REFERENCE: u32 = 0x0104B818;
        const OBJECT: u32 = 0x016EC77C;
        const DELTA: u32 = 0x016EC780;
        const CALLBACK: u32 = 0x009FD380;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let m = (lf_checker_rt::relocated(MASTER) as *const f32).read_unaligned();
        let r = (lf_checker_rt::relocated(REFERENCE) as *const f32).read_unaligned();
        (lf_checker_rt::relocated(DELTA) as *mut f32).write_unaligned(sub(m, r));
        let obj = (lf_checker_rt::relocated(OBJECT) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, obj, arg, lf_checker_rt::relocated(CALLBACK))
    }
});
