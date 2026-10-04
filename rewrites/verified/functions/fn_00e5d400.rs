// original: 0x00e5d400 construct_pool8_e5d400
/// Constructs the eight entries of a fixed object pool by invoking the
/// entry constructor on each slot in address order; returns the last
/// constructor's answer.
export!(cdecl, rw_e5d400() -> u32 {
    const COUNT: u32 = 8;
    const STRIDE: u32 = 0x108;
    let mut answer = 0;
    for i in 0..COUNT {
        let slot = relocated(0x019ACC50).wrapping_add(i.wrapping_mul(STRIDE));
        answer = callee_thiscall!(2, u32, slot);
    }
    answer
});
