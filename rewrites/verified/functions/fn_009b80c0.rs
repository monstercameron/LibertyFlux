// original: 0x009B80C0 clear_ten_camera_slots
/// Initializes the per-slot camera instruction state and calls one helper for each of ten records separated by 0x84 bytes. The sibling performs its all-ones marker write after the loop. The call trace is compared for every slot.
lf_checker_rt::export!(thiscall, rw_009b80c0(this: u32) -> u32 {
    const SLOT_COUNT: u32 = 10;
    const SLOT_STRIDE: u32 = 0x84;
    const LAST_RESULT: u32 = 0x52C;
    
    let mut index = 0u32;
    while index < SLOT_COUNT {
        let slot_base = this.wrapping_add(index.wrapping_mul(SLOT_STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, slot_base);
        index += 1;
    }
    unsafe { (this.wrapping_add(LAST_RESULT) as *mut u32).write_unaligned(u32::MAX); }
    
    0
});
