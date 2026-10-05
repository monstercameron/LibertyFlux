// original: 0x00b1fc70 store_grid_coords (proposed)

/// Stores three masked coordinates through three out-pointers, returns last.
///
/// Arguments are six stack words (stdcall, callee pops 24 bytes): three
/// values followed by three destination pointers. The first two values are
/// masked to 5 bits, the third to 3 bits, and each is stored through its
/// pointer. Returns the third out-pointer unchanged.
lf_checker_rt::export!(stdcall, rw_00b1fc70(v0: u32, v1: u32, v2: u32, out0: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        const MASK5: u32 = 0x1f;
        const MASK3: u32 = 0x07;
        (out0 as *mut u32).write_unaligned(v0 & MASK5);
        (out1 as *mut u32).write_unaligned(v1 & MASK5);
        (out2 as *mut u32).write_unaligned(v2 & MASK3);
    }
    out2
});
