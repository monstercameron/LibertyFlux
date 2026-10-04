// original: 0x009FD350 frag_schedule_side_b (proposed)

/// Stash the time delta for side B and schedule its callback.
///
/// Subtracts the side-B reference time global from the master time global
/// (original operand order: master minus reference), stores the difference
/// in the side-B delta global, and invokes the scheduler callee on the
/// side-B object with (`arg`, callback), where the callback is the shared
/// side-B handler. Returns the scheduler's answer.
///
/// Original: 0x009FD350 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FD350(arg: u32) -> u32 {
    unsafe {
        const MASTER: u32 = 0x00FE8B08;
        const REFERENCE: u32 = 0x0104B878;
        const OBJECT: u32 = 0x016EC7A8;
        const DELTA: u32 = 0x016EC7AC;
        const CALLBACK: u32 = 0x009FD3D0;
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
