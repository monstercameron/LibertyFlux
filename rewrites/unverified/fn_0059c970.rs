// original: 0x0059C970 CNetBlenderDummyPed::vf14

/// Apply a blend update to the entity stored in this dummy.
///
/// The entity pointer is at `this + 0xC0`. The second stack argument selects
/// the branch by its low byte, while the first stack argument is forwarded
/// unchanged. Both branches call the entity's virtual slot at `+0x08` with
/// `(first_argument, branch_value, 0)`; a nonzero low byte also calls the
/// blend update helper and then virtual slot `+0xB4` with `1`. The last
/// called helper supplies the return value. This is a thiscall with two
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_0059c970(this: u32, first_argument: u32, branch_word: u32) -> u32 {
    const ENTITY_POINTER: u32 = 0xC0;
    const BRANCH_FALSE: u32 = 0;
    const BRANCH_TRUE: u32 = 1;
    const ZERO: u32 = 0;

    unsafe {
        let entity = ((this.wrapping_add(ENTITY_POINTER)) as *const u32).read_unaligned();
        let branch_value = u32::from((branch_word as u8) != 0);
        let initial_result = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            entity,
            first_argument,
            branch_value,
            ZERO
        );

        if branch_value == BRANCH_FALSE {
            return initial_result;
        }

        let _ = lf_checker_rt::callee_thiscall!(2, u32, entity);
        lf_checker_rt::callee_thiscall!(3, u32, entity, BRANCH_TRUE)
    }
});
