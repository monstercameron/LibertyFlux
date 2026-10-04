// original: 0x008ed4f0 NativeImpl_LOAD_PATH_NODES_IN_AREA
/// Store a region request and kick the node loader.
///
/// Writes the flag byte and the four bound floats of slot `idx`, then
/// calls the loader worker (stubbed by the checker) with a constant 1,
/// returning its answer.
export!(thiscall, rw_008ed4f0(
    this: *mut u8,
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    idx: u32,
) -> u32 {
    unsafe {
        *(this.add((0x1A8C + idx) as usize) as *mut u8) = 1;
        *(this.add((0x1A90 + idx * 4) as usize) as *mut f32) = f0;
        *(this.add((0x1A9C + idx * 4) as usize) as *mut f32) = f1;
        *(this.add((0x1AA8 + idx * 4) as usize) as *mut f32) = f2;
        *(this.add((0x1AB4 + idx * 4) as usize) as *mut f32) = f3;
        callee_thiscall!(1, u32, this as u32, 1)
    }
});
