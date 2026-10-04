// original: 0x009f5330 table_zero_32
/// Zero 32 consecutive network-table dwords. The original clears a run of 32
/// counters/pointers with individual stores and returns zero.
export!(cdecl, rw_009f5330() -> u32 {
    let base = global::<u32>(0x012B_61D0);
    let mut i = 0usize;
    while i < 32 {
        // SAFETY: worker maps the original image; range is .data.
        unsafe { base.add(i).write(0) };
        i += 1;
    }
    0
});
