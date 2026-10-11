// original: 0x005E89C0 CPedIntelligenceNY::vf6
const ACTION_FLAG_OFFSET: u32 = 0x26c;
const ACTION_OBJECT_OFFSET: u32 = 0x224;
const RESPONSE_OBJECT_OFFSET: u32 = 0x78;
const PRIMARY_ACTION_CODE: u32 = 0x116;
const FALLBACK_ACTION_CODE: u32 = 0x12b;

/// Checks whether the second stack argument's action flag permits the
/// primary action. When enabled, it calls the action handler at the pointer
/// stored at offset `0x224` plus `0x44` with code `0x116`; a zero result
/// returns true. Otherwise it calls the response handler stored at offset
/// `0x78` in the first stack argument with code `0x12b` and returns whether
/// that result is nonzero. ECX is unused. This thiscall method pops both
/// stack words and returns a 32-bit boolean in EAX.
lf_checker_rt::export!(thiscall, rw_005e89c0(_this: u32, response_context: u32, action_context: u32) -> u32 {
    let action_flag = unsafe {
        (action_context.wrapping_add(ACTION_FLAG_OFFSET) as *const u8).read()
    };
    if action_flag & 1 != 0 {
        let action_object = unsafe {
            (action_context.wrapping_add(ACTION_OBJECT_OFFSET) as *const u32).read_unaligned()
        };
        let action_handler = action_object.wrapping_add(0x44);
        let first_result = lf_checker_rt::callee_thiscall!(1, u32, action_handler, PRIMARY_ACTION_CODE);
        if first_result == 0 {
            return 1;
        }
    }

    let response_handler = unsafe {
        (response_context.wrapping_add(RESPONSE_OBJECT_OFFSET) as *const u32).read_unaligned()
    };
    let fallback_result =
        lf_checker_rt::callee_thiscall!(2, u32, response_handler, FALLBACK_ACTION_CODE);
    u32::from(fallback_result != 0)
});
