// original: 0x00B31420 classify_ped_task_state

/// Return one when the byte tag at +0xa60 equals two. For other tags, return
/// two when bit two at +0x26c is set and zero otherwise. The input is one
/// caller-cleaned object pointer and the result is an integer classification.
lf_checker_rt::export!(cdecl, rw_b31420(object: u32) -> u32 {
    unsafe {
        const TAG_OFFSET: u32 = 0x0a60;
        const FLAGS_OFFSET: u32 = 0x026c;
        let tag = (object.wrapping_add(TAG_OFFSET) as *const u8).read();
        if tag == 2 {
            1
        } else {
            let flags = (object.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned();
            if flags & 4 != 0 { 2 } else { 0 }
        }
    }
});
