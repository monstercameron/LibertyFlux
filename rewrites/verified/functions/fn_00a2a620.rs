// original: 0x00a2a620 CPlayerPed::vf47

/// Ask the global player context for a lookup object, then forward three of
/// this method's stack words and the ped pointer through its setup helper.
/// Store the helper result at `this+0x6c`, pass the final stack word to the
/// follow-up helper, and return the stored result. ECX is `this`; five words
/// are callee-cleaned. The first four stack words go to the setup helper and the fifth goes to the follow-up helper.
lf_checker_rt::export!(thiscall, rw_a2a620(this: u32, first: u32, second: u32, third: u32, setup_tail: u32, final_arg: u32) -> u32 {
    const PLAYER_CONTEXT: u32 = 0x018b62dc;
    const RESULT_FIELD: u32 = 0x006c;
    let context = unsafe { lf_checker_rt::global::<u32>(PLAYER_CONTEXT).read_unaligned() };
    let lookup = unsafe { lf_checker_rt::callee_thiscall!(1, u32, context) };
    let result = if lookup != 0 {
        unsafe { lf_checker_rt::callee_thiscall!(2, u32, lookup, this, 0, first, second, third, setup_tail) }
    } else {
        0
    };
    unsafe { ((this + RESULT_FIELD) as *mut u32).write_unaligned(result) };
    let _ignored = unsafe { lf_checker_rt::callee_thiscall!(3, u32, result, final_arg) };
    unsafe { ((this + RESULT_FIELD) as *const u32).read_unaligned() }
});
