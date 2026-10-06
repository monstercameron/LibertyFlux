// original: 0x009A66E0 SFX_

// Matches NUL-terminated names longer than 4 bytes against the "SFX_" tag.
//
// stdcall, one stack word `name` pointing at a NUL-terminated byte string.
// Returns 1 when the string is longer than 4 bytes (the length bound is a
// SIGNED compare, `jle`, though a length is never negative) and its first
// 4 bytes equal the tag at `TAG` (compared by callee 1, cdecl with
// (`name`, `TAG`, 4), which answers 0 on equality); otherwise returns 0.
lf_checker_rt::export!(stdcall, rw_009a66e0(name: u32) -> u32 {
    unsafe {
        const TAG_CMP: u32 = 1;
        const TAG: u32 = 0x00E914DC;
        const TAG_LEN: u32 = 4;
        let mut len: u32 = 0;
        loop {
            let b = (name.wrapping_add(len) as *const u8).read();
            if b == 0 {
                break;
            }
            len = len.wrapping_add(1);
        }
        if (len as i32) > (TAG_LEN as i32) {
            let tag = lf_checker_rt::relocated(TAG);
            let diff: u32 = lf_checker_rt::callee_cdecl!(TAG_CMP, u32, name, tag, TAG_LEN);
            if diff == 0 {
                return 1;
            }
        }
    }
    0
});
