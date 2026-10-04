// original: 0x00c68c50 subarray_probe_negated
// Probe the embedded slot array at +0x404 and return the negated answer:
// 1 when the probe reports 0, else 0.
export!(thiscall, rw_00c68c50(obj: u32) -> u32 {
    unsafe {
        let a: u32 = callee_thiscall!(1, u32, obj.wrapping_add(0x404));
        if a & 0xff == 0 {
            1
        } else {
            0
        }
    }
});
