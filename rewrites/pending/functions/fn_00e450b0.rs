// original: 0x00e450b0 single_publish_done
// single publish with done flag.
// Publishes (arg, 1) through the target at this+0x48, then sets the byte at
// this+0x450. Returns the publish helper's answer.
export!(thiscall, rw_00e450b0(this_obj: u32, arg: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x48;
        const DONE_OFF: u32 = 0x450;
        let target = *((this_obj.wrapping_add(TARGET_OFF)) as *const u32);
        let ans: u32 = callee_thiscall!(1, u32, target, arg, 1);
        *((this_obj.wrapping_add(DONE_OFF)) as *mut u8) = 1;
        ans
    }
});
