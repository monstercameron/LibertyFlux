// original: 0x00e45020 slot_array_build_register_a
// counted slot array build and register.
// Allocates count*16 bytes where count is the byte at this+0x418, constructs
// one slot per count entry, then registers the array; on success the handle
// is published and the buffer released with the done flag set, otherwise the
// buffer is just released. The multiply-overflow guard in the original cannot
// fire for a byte-sized count, so the size is exactly count*16. Returns the
// release helper's answer.
export!(thiscall, rw_00e45020(this_obj: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x418;
        const PARAM_OFF: u32 = 0x58;
        const TARGET_OFF: u32 = 0x48;
        const DONE_OFF: u32 = 0x44f;
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
