// original: 0x00ade040 CRenderPhaseDrawScene::vf3

/// Rebuild the draw-scene phase's cached pass, or adopt the pass of the
/// chained phase.
///
/// `this` is the phase object. When the mode byte at `this+0x940` is
/// zero, the pass loader callee (id 1) runs and its result is stored at
/// `this+0x938`, the six-argument setup callee (id 2) runs with
/// (`this+0x900`, `this+0x8f8`, `this+0x8fc`, `this`, 1, 0), the cached
/// pass is forwarded to the bind callee (id 3), the finalize callee
/// (id 4) runs, and the phase is submitted through the queue callee
/// (id 5). When the mode byte is nonzero, the cached pass is copied
/// from the chained phase (`this+0x944`, word at `+0x938`) and only
/// the queue callee runs.
///
/// Edge cases: any nonzero mode byte takes the adopt path; the loader
/// result is stored before the setup call so the bind call forwards
/// exactly what was stored.
///
/// Original: thiscall, no stack arguments, five direct callees, returns
/// nothing.
lf_checker_rt::export!(thiscall, rw_00ade040(this: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x940;
        const CACHED: u32 = 0x938;
        const CHAIN: u32 = 0x944;
        const SETUP_A: u32 = 0x900;
        const SETUP_B: u32 = 0x8f8;
        const SETUP_C: u32 = 0x8fc;
        const LOADER_OBJ: u32 = 0x01614C90;
        const QUEUE_OBJ: u32 = 0x01394D60;
        let loader = lf_checker_rt::relocated(LOADER_OBJ);
        let queue = lf_checker_rt::relocated(QUEUE_OBJ);
        let mode = ((this + MODE) as *const u8).read();
        if mode == 0 {
            let pass: u32 = lf_checker_rt::callee_thiscall!(1, u32, loader);
            ((this + CACHED) as *mut u32).write_unaligned(pass);
            lf_checker_rt::callee_cdecl!(
                2, u32, this + SETUP_A, this + SETUP_B, this + SETUP_C, this, 1, 0
            );
            let cached = ((this + CACHED) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(3, u32, cached);
            lf_checker_rt::callee_cdecl!(4, u32,);
            lf_checker_rt::callee_thiscall!(5, u32, queue, this);
        } else {
            let chained = ((this + CHAIN) as *const u32).read_unaligned();
            let adopted = ((chained + CACHED) as *const u32).read_unaligned();
            ((this + CACHED) as *mut u32).write_unaligned(adopted);
            lf_checker_rt::callee_thiscall!(5, u32, queue, this);
        }
    }
    0
});
