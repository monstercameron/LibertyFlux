// original: 0x00e450d0 slot_array_build_register_b
// counted slot array build and register, second channel.
// Same shape as 0x00e45020 with the count at this+0x40c, the register
// parameter at this+0x54 and the done flag at this+0x44e. Returns the
// release helper's answer.
export!(thiscall, rw_00e450d0(this_obj: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x40c;
        const PARAM_OFF: u32 = 0x54;
        const TARGET_OFF: u32 = 0x48;
        const DONE_OFF: u32 = 0x44e;
        const SLOT: u32 = 0x10;
        let count = *((this_obj.wrapping_add(COUNT_OFF)) as *const u8) as u32;
        let buf: u32 = callee_cdecl!(1, u32, count.wrapping_mul(SLOT));
        if buf != 0 {
            let mut left: i32 = (count as i32).wrapping_sub(1);
            let mut slot = buf;
            if left >= 0 {
                loop {
                    callee_thiscall!(2, u32, slot);
                    slot = slot.wrapping_add(SLOT);
                    left = left.wrapping_sub(1);
                    if left < 0 {
                        break;
                    }
                }
            }
        }
        let param = *((this_obj.wrapping_add(PARAM_OFF)) as *const u32);
        let status: u32 = callee_thiscall!(3, u32, this_obj, param, count, buf);
        if (status as i32) > 0 {
            let target = *((this_obj.wrapping_add(TARGET_OFF)) as *const u32);
            callee_thiscall!(4, u32, target, buf, status);
            let freed: u32 = callee_cdecl!(5, u32, buf);
            *((this_obj.wrapping_add(DONE_OFF)) as *mut u8) = 1;
            freed
        } else {
            callee_cdecl!(5, u32, buf)
        }
    }
});
