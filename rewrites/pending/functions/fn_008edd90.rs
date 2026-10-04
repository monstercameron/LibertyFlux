// original: 0x008edd90 NativeImpl_RELEASE_PATH_NODES
/// Clear the request flag of slot `idx`. Returns `idx`.
export!(thiscall, rw_008edd90(this: *mut u8, idx: u32) -> u32 {
    unsafe {
        *(this.add((0x1A8C + idx) as usize) as *mut u8) = 0;
        idx
    }
});
