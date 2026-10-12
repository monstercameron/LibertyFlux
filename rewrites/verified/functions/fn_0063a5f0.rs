// original: 0x0063A5F0 rage::ptxTriggerEvent::resolve_name_entry

/// Resolve the trigger's name entry unless the state word at `this + 0x64`
/// is already set. The lookup receives a relocated image name in ECX, a
/// context selected from `this + 0x60` and its `+0xF8` word (or a relocated
/// default token) in EDX, and the literal one as its stack argument. Its
/// result is passed in ECX to the tail helper together with this function's
/// original stack argument. `this` is in ECX; the method has one stack word.
lf_checker_rt::export!(thiscall, rw_0063A5F0(this: u32, stack_argument: u32) -> u32 {
    const STATE_WORD: u32 = 0x64;
    const NAME_CONTEXT: u32 = 0x60;
    const CONTEXT_NAME: u32 = 0xF8;
    const DEFAULT_NAME_TOKEN: u32 = 0x00F1C06F;
    const NAME_FIELD: u32 = 0x00F98B5C;
    const LOOKUP_NAME: u32 = 1;
    const RESOLVE_ENTRY: u32 = 2;

    unsafe {
        let state = (this.wrapping_add(STATE_WORD) as *const u32).read_unaligned();
        if state != 0 {
            return 0;
        }

        let context_pointer = (this.wrapping_add(NAME_CONTEXT) as *const u32).read_unaligned();
        let candidate_name = if context_pointer == 0 {
            0
        } else {
            (context_pointer.wrapping_add(CONTEXT_NAME) as *const u32).read_unaligned()
        };
        let lookup_context = if candidate_name == 0 {
            lf_checker_rt::relocated(DEFAULT_NAME_TOKEN)
        } else {
            candidate_name
        };
        let resolved_name = lf_checker_rt::callee_fastcall!(
            LOOKUP_NAME,
            u32,
            lf_checker_rt::relocated(NAME_FIELD),
            lookup_context,
            1u32,
        );
        lf_checker_rt::callee_thiscall!(
            RESOLVE_ENTRY,
            u32,
            resolved_name,
            stack_argument,
        )
    }
});
