// original: 0x00D26250 link_vehicle_anim
/// Selects an animation tag for a cdecl action code, consulting the lookup helper on three cases,
/// then links the source and context through two helper calls. The action comparison is unsigned,
/// values outside zero through eight use the default tag, and the final helper receives the
/// source, the exact 1.0 float bits, the link result and the selected tag. The intermediate call
/// passes a pointer into its own frame; the proof skips that address and compares the five input
/// words it points to.
lf_checker_rt::export!(cdecl, rw_00d26250(source: u32, animation_context: u32, action_code: u32, source_tag: u32, source_flags: u32) -> u32 {
    const MAX_ACTION_CODE: u32 = 8;
    const DEFAULT_TAG: u32 = 0x13f;
    const ACTION_ONE_QUERY: u32 = 0x141;
    const ACTION_ONE_MISS: u32 = 0x13f;
    const ACTION_ONE_TAG: u32 = 0x141;
    const ACTION_TWO_TAG: u32 = 0x140;
    const ACTION_THREE_QUERY: u32 = 0x142;
    const ACTION_THREE_MISS: u32 = 0x140;
    const ACTION_THREE_TAG: u32 = 0x142;
    const ACTION_FOUR_QUERY: u32 = 0x14e;
    const ACTION_FOUR_MISS: u32 = 0x14c;
    const ACTION_FOUR_TAG: u32 = 0x14e;
    const ACTION_FIVE_TAG: u32 = 0x14c;
    const ACTION_SIX_TAG: u32 = 0x16d;
    const ACTION_SEVEN_TAG: u32 = 0x16f;
    const ONE_BITS: u32 = 0x3f800000;
    let action_tag = if action_code > MAX_ACTION_CODE {
        DEFAULT_TAG
    } else {
        match action_code {
            0 => DEFAULT_TAG,
            1 => {
                let found = unsafe { lf_checker_rt::callee_cdecl!(1, u32, animation_context, ACTION_ONE_QUERY) };
                if found == 0 { ACTION_ONE_MISS } else { ACTION_ONE_TAG }
            }
            2 => ACTION_TWO_TAG,
            3 => {
                let found = unsafe { lf_checker_rt::callee_cdecl!(1, u32, animation_context, ACTION_THREE_QUERY) };
                if found == 0 { ACTION_THREE_MISS } else { ACTION_THREE_TAG }
            }
            4 => DEFAULT_TAG,
            5 => ACTION_FIVE_TAG,
            6 => {
                let found = unsafe { lf_checker_rt::callee_cdecl!(1, u32, animation_context, ACTION_FOUR_QUERY) };
                if found == 0 { ACTION_FOUR_MISS } else { ACTION_FOUR_TAG }
            }
            7 => ACTION_SIX_TAG,
            8 => ACTION_SEVEN_TAG,
            _ => DEFAULT_TAG,
        }
    };
    let frame_words = [0u32, source, animation_context, action_code, source_tag, source_flags];
    let frame_pointer = frame_words.as_ptr() as u32;
    let link_result = unsafe {
        lf_checker_rt::callee_cdecl!(2, u32, animation_context, frame_pointer, source_flags, source_tag, 0, 0, 0, 0)
    };
    unsafe {
        let _ = lf_checker_rt::callee_cdecl!(3, u32, source, ONE_BITS, link_result, action_tag);
    }
    source
});
