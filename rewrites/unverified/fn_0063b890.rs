// original: 0x0063B890 rage::ptxModel::vf5

/// Write the supplied value to the drawable-pool pointer slot at offset `0x170` and return it unchanged.
lf_checker_rt::export!(thiscall, rw_0063b890(this: *mut u8, value: u32) -> u32 {
    unsafe {
        const DRAWABLE_POOL_PTR: usize = 0x170;
        (this.add(DRAWABLE_POOL_PTR) as *mut u32).write_unaligned(value);
    }
    value
});
