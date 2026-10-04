// original: 0x009fb080 playstats

/// Initialise the playstats collector: zero its counters, create its two
/// synchronisation objects, and spawn its worker thread.
///
/// Takes no arguments and no register inputs (cdecl). Selects one of two
/// 2048-byte banks from `ACTIVE_SET` into `SET_BASE`, zeroes the eight counter
/// words `GEN_FLAG`..`SPAN_END`, sets the two `TIMEOUT_*` words to all-ones,
/// and clears `READY_BYTE`; then runs callee 0 (cdecl, `CFG_PTR` and
/// `SET_OBJ`), callee 1 (cdecl, no arguments) and callee 2/3 (thiscall on
/// `BANK_OBJ`/`AUX_OBJ`, the same entry point). It zeroes `DST_A`/`DST_B` and
/// `DONE_BYTE`, then creates object A via callee 4 into `OBJ_A` and object B
/// via callee 5 into `OBJ_B`.
///
/// When both objects are nonzero it spawns the worker via callee 6 (cdecl,
/// entry `WORKER_FN`, flags `SPAWN_FLAGS`, and name/config pointers),
/// records the handle in `THREAD_HANDLE`, and: if the handle equals the
/// `BAD_HANDLE` sentinel it reloads both objects and goes to teardown;
/// otherwise it runs callee 7 (cdecl, object A) and returns `BAD_HANDLE`
/// (the handle can no longer equal it). Teardown loads `BAD_HANDLE` and
/// returns it at once when it differs from `THREAD_HANDLE`; otherwise it
/// releases object A via callee 8 and object B via callee 9 (each only when
/// nonzero), clearing their slots, and returns the last release answer
/// (`BAD_HANDLE` when neither released).
lf_checker_rt::export!(cdecl, rw_009fb080() -> u32 {
    unsafe {
        const ACTIVE_SET: u32 = 0x012B_9000;
        const SET_BASE: u32 = 0x012B_9084;
        const BANK_LO: u32 = 0x012B_8000;
        const BANK_STRIDE_SHIFT: u32 = 11;
        const GEN_FLAG: u32 = 0x012B_9004;
        const SPAN_WORDS: u32 = 8;
        const TIMEOUT_A: u32 = 0x0103_B584;
        const TIMEOUT_B: u32 = 0x0103_B588;
        const READY_BYTE: u32 = 0x012B_9038;
        const SET_OBJ: u32 = 0x012B_9038;
        const CFG_PTR: u32 = 0x00E9_9644;
        const BANK_OBJ: u32 = 0x012B_9088;
        const AUX_OBJ: u32 = 0x012B_9100;
        const DST_A: u32 = 0x012B_9078;
        const DST_B: u32 = 0x012B_907C;
        const DONE_BYTE: u32 = 0x012B_90FC;
        const OBJ_A: u32 = 0x012B_9584;
        const OBJ_B: u32 = 0x012B_9C3C;
        const THREAD_HANDLE: u32 = 0x012B_9080;
        const BAD_HANDLE: u32 = 0x00F1_C040;
        const WORKER_FN: u32 = 0x009F_CC50;
        const SPAWN_FLAGS: u32 = 0x6000;
        const WORKER_NAME: u32 = 0x00E9_9650;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a)).write_unaligned(v) }
        }

        let set = rd32(ACTIVE_SET);
        // The worker relocates the image: every absolute address the original
        // forms as a value must be relocated too (plain constants stay raw).
        let bank_lo = lf_checker_rt::relocated(BANK_LO);
        let set_obj = lf_checker_rt::relocated(SET_OBJ);
        let cfg_ptr = lf_checker_rt::relocated(CFG_PTR);
        let bank_obj = lf_checker_rt::relocated(BANK_OBJ);
        let aux_obj = lf_checker_rt::relocated(AUX_OBJ);
        let worker_fn = lf_checker_rt::relocated(WORKER_FN);
        let worker_name = lf_checker_rt::relocated(WORKER_NAME);
        wr32(SET_BASE, set.wrapping_shl(BANK_STRIDE_SHIFT).wrapping_add(bank_lo));
        for i in 0..SPAN_WORDS {
            wr32(GEN_FLAG.wrapping_add(i.wrapping_mul(4)), 0);
        }
        wr32(TIMEOUT_A, 0xFFFF_FFFF);
        wr32(TIMEOUT_B, 0xFFFF_FFFF);
        lf_checker_rt::global::<u8>(READY_BYTE).write(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(0, u32, cfg_ptr, set_obj);
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, bank_obj);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, aux_obj);
        wr32(DST_A, 0);
        wr32(DST_B, 0);
        lf_checker_rt::global::<u8>(DONE_BYTE).write(0);
        let obj_a: u32 = lf_checker_rt::callee_cdecl!(4, u32, 0u32);
        wr32(OBJ_A, obj_a);
        let obj_b: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0u32);
        wr32(OBJ_B, obj_b);
        let mut result = rd32(BAD_HANDLE);
        let (mut rel_a, mut rel_b) = (obj_a, obj_b);
        if obj_a != 0 && obj_b != 0 {
            let handle: u32 = lf_checker_rt::callee_cdecl!(
                6, u32, worker_fn, 0u32, SPAWN_FLAGS, 0u32, worker_name, 1u32, 0u32
            );
            wr32(THREAD_HANDLE, handle);
            if handle != rd32(BAD_HANDLE) {
                let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, rd32(OBJ_A));
                return result;
            }
            rel_a = rd32(OBJ_A);
            rel_b = rd32(OBJ_B);
        }
        if result != rd32(THREAD_HANDLE) {
            return result;
        }
        if rel_a != 0 {
            result = lf_checker_rt::callee_cdecl!(8, u32, rel_a);
            rel_b = rd32(OBJ_B);
            wr32(OBJ_A, 0);
        }
        if rel_b != 0 {
            result = lf_checker_rt::callee_cdecl!(9, u32, rel_b);
            wr32(OBJ_B, 0);
        }
        result
    }
});
