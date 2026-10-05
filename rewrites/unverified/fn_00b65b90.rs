// original: 0x00B65B90 veh_visit_positive_11
/// Forward `(a0,1,0)`, visit the 11 positive slots, clear `[this]`.
///
/// Calls the guarded forwarder (stubbed, thiscall/3) with `(a0, 1, 0)`, then
/// scans the 11 dwords at `this+0x24+k*0xc` (k = 0..10), invoking the visitor
/// (stubbed, thiscall/0) on each slot holding a positive (signed) value, and
/// zeroes `[this]`. Thiscall, one stack word. No meaningful return value.
export!(thiscall, rw_00b65b90(this: u32, a0: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x24;
        const STRIDE: u32 = 0xc;
        const COUNT: u32 = 11;
        let _: u32 = callee_thiscall!(1, u32, this, a0, 1, 0);
        let mut k = 0u32;
        while k < COUNT {
            let slot = this + BASE + k * STRIDE;
            if (slot as *const i32).read_unaligned() > 0 {
                let _: u32 = callee_thiscall!(2, u32, slot);
            }
            k += 1;
        }
        (this as *mut u32).write_unaligned(0);
        0
    }
});
