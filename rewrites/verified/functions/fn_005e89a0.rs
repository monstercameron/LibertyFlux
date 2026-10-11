// original: 0x005E89A0 CPedIntelligenceNY::vf5
const PED_RESPONSE_STATE_OFFSET: u32 = 0x20;
const PED_RESPONSE_CONTEXT_OFFSET: u32 = 0x30;
const PED_RESPONSE_FLOAT_FILE_VA: u32 = 0x00FE8C20;

/// Reads a response-state pointer at offset `0x20` in the stack argument,
/// adds `0x30`, and calls the response helper with the resulting context in
/// ECX, the original record and zero on the stack, and the configured float
/// in XMM1. It returns the helper's EAX. The contract transports XMM1 through
/// a third stack word and compares its low 32-bit lane; the original caller
/// cleans its one stack argument.
lf_checker_rt::export!(thiscall, rw_005e89a0(_this: u32, action_record: u32) -> u32 {
    let response_state = unsafe {
        (action_record.wrapping_add(PED_RESPONSE_STATE_OFFSET) as *const u32)
            .read_unaligned()
    };
    let response_context = response_state.wrapping_add(PED_RESPONSE_CONTEXT_OFFSET);
    let response_float = unsafe {
        lf_checker_rt::global::<u32>(PED_RESPONSE_FLOAT_FILE_VA).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(1, u32, response_context, action_record, 0, response_float)
});
