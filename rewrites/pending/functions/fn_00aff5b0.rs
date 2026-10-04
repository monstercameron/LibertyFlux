// original: 0x00aff5b0 iterate_slots_callback
/// Invoke the slot callback for each populated slot in the window selected
/// by the shared index. Slots flagged 0x80 and null entries are skipped.
export!(cdecl, rw_00aff5b0(arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let index = (*global::<u32>(0x1173604) & 0x1f) as i32;
        let obj = *global::<u32>(0x12e22a4) as *const u32;
        let base = *obj;
        let flag_base = *obj.add(1);
        let count = *obj.add(2) as i32;
        let stride = *obj.add(3) as i32;
        // Signed floor-division by 32, as the original's cdq/and/sar does.
        let floor32 = |product: i32| {
            product.wrapping_add(if product < 0 { 0x1f } else { 0 }) >> 5
        };
        let end = floor32(count.wrapping_mul(index.wrapping_add(1)));
        let mut slot = floor32(count.wrapping_mul(index));
        while slot < end {
            let flag = *((flag_base.wrapping_add(slot as u32)) as *const u8);
            if flag & 0x80 == 0 {
                let entry = (stride.wrapping_mul(slot) as u32).wrapping_add(base);
                if entry != 0 {
                    callee_cdecl!(1, u32, entry, arg0, arg1, arg2);
                }
            }
            slot = slot.wrapping_add(1);
        }
        0
    }
});
