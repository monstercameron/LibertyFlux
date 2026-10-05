// original: 0x0061D730 net_copy_record_list

/// Copy a caller record list into this object.
///
/// Stores `a0`/`a1` in the header, copies `count` 8-byte records from
/// `src` to `this+8` (nothing when `count` is zero; the contract keeps
/// it small and non-negative since a negative count never terminates),
/// then records the count at `this+0x10C` and `a4` at `this+0x108`.
/// Returns `a4`.
/// Original: 0x0061D730 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_0061D730(this: u32, a0: u32, a1: u32, src: u32, count: u32, a4: u32) -> u32 {
    unsafe {
        const RECORD_BYTES: u32 = 8;
        ((this) as *mut u32).write_unaligned(a0);
        ((this + 4) as *mut u32).write_unaligned(a1);
        let n = count as i32;
        if n > 0 {
            for i in 0..n as u32 {
                let v = ((src + i * RECORD_BYTES) as *const u64).read_unaligned();
                ((this + 8 + i * RECORD_BYTES) as *mut u64).write_unaligned(v);
            }
        }
        ((this + 0x10C) as *mut u32).write_unaligned(count);
        ((this + 0x108) as *mut u32).write_unaligned(a4);
        a4
    }
});
