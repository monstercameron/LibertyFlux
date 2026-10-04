// original: 0x008ece70 NativeImpl_HAVE_REQUESTED_PATH_NODES_BEEN_LOADED
/// Forward one row of region bounds to the loader worker.
///
/// Reads the four floats of slot `idx` and passes them to the engine
/// routine (stubbed by the checker), returning its answer.
export!(thiscall, rw_008ece70(this: *const u8, idx: u32) -> u32 {
    unsafe {
        let f = |off: u32| {
            *(this.add((off + idx * 4) as usize) as *const u32)
        };
        callee_thiscall!(
            1,
            u32,
            this as u32,
            f(0x1A90),
            f(0x1A9C),
            f(0x1AA8),
            f(0x1AB4)
        )
    }
});
