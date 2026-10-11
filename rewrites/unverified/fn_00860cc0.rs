// original: 0x00860CC0 physics_dispatch_child_callbacks

/// Invokes the optional callback at virtual slot `+0x18` on each of two child
/// objects. A nonzero low return byte triggers that object's slot `+0x0c`.
/// The second slot `+0x0c` call is a tail call, so its full return value is
/// returned to the caller. When its predicate byte is zero, the full predicate
/// return register is left untouched through the ordinary return path.
lf_checker_rt::export!(thiscall, rw_00860cc0(this: u32) -> u32 {
    unsafe {
        const CHILD_CONTAINER: u32 = 0x0010;
        const FIRST_CHILD: u32 = 0x0020;
        const SECOND_CHILD: u32 = 0x001c;
        const TEST_SLOT_FIRST: u32 = 1;
        const ACTION_SLOT_FIRST: u32 = 2;
        const TEST_SLOT_SECOND: u32 = 3;
        const ACTION_SLOT_SECOND: u32 = 4;

        #[inline(always)]
        unsafe fn read_word(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }

        let children = read_word(this, CHILD_CONTAINER);
        let first = read_word(children, FIRST_CHILD);
        let first_result = lf_checker_rt::callee_thiscall!(TEST_SLOT_FIRST, u32, first);
        if first_result as u8 != 0 {
            let _ignored =
                lf_checker_rt::callee_thiscall!(ACTION_SLOT_FIRST, u32, first);
        }

        let second = read_word(children, SECOND_CHILD);
        let second_result = lf_checker_rt::callee_thiscall!(TEST_SLOT_SECOND, u32, second);
        if second_result as u8 != 0 {
            return lf_checker_rt::callee_thiscall!(ACTION_SLOT_SECOND, u32, second);
        }
        second_result
    }
});
