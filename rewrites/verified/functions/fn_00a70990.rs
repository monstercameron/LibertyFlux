// original: 0x00a70990 CTaskComplexPlayerPlaceCarBomb::vf20

/// State hook of the place-car-bomb task: either arms the bomb timer or
/// refreshes the task's link, and returns the linked child in most cases.
///
/// When the started byte at `this + STARTED` is non-zero, returns the child
/// at `this + CHILD` at once. When the state word at `this + STATE` is
/// non-zero, returns the child as well unless the latched byte at
/// `this + LATCHED` has bit 0 clear, in which case the timer block routine
/// (callee 2) runs first with `this + TIMER` in ecx and `TIMER_ARG` on the
/// stack.
///
/// Otherwise (fresh task): stamps `FRESH_MARK` into the half-word at
/// `this + MARK`, loads the child into a local, and checks the child's flag
/// byte at `child + FLAG`. If bit 0 is already set, returns null. Else runs
/// the child's link virtual at slot `LINK_SLOT` (callee 1, reached through
/// the child's vtable, `child` in ecx, words `arg, 1, 0`); a zero low byte
/// in its answer returns the child, otherwise sets bit 1 of the child's
/// flag word and returns null.
///
/// Original: thiscall, one stack word (`arg`), callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a70990(this: u32, arg: u32) -> u32 {
    unsafe {
        const STARTED: u32 = 0x19;
        const STATE: u32 = 0x14;
        const CHILD: u32 = 8;
        const LATCHED: u32 = 0x26;
        const TIMER: u32 = 0x1c;
        const TIMER_ARG: u32 = 0x18;
        const MARK: u32 = 0x18;
        const FRESH_MARK: u16 = 0x0101;
        const FLAG: u32 = 0x0c;
        const LINK_SLOT: u32 = 0x14;
        const TIMER_BLOCK: u32 = 2;

        let started = ((this as *const u8).wrapping_byte_offset(STARTED as isize)).read();
        if started != 0 {
            return ((this as *const u32).wrapping_byte_offset(CHILD as isize)).read_unaligned();
        }
        let state = ((this as *const u32).wrapping_byte_offset(STATE as isize)).read_unaligned();
        if state != 0 {
            let latched = ((this as *const u8).wrapping_byte_offset(LATCHED as isize)).read();
            if latched & 1 == 0 {
                lf_checker_rt::callee_thiscall!(
                    TIMER_BLOCK, u32, this.wrapping_add(TIMER), TIMER_ARG);
            }
            return ((this as *const u32).wrapping_byte_offset(CHILD as isize)).read_unaligned();
        }
        let child = ((this as *const u32).wrapping_byte_offset(CHILD as isize)).read_unaligned();
        ((this as *mut u16).wrapping_byte_offset(MARK as isize)).write_unaligned(FRESH_MARK);
        let flag = ((child as *const u8).wrapping_byte_offset(FLAG as isize)).read();
        if flag & 1 != 0 {
            return 0;
        }
        let vtable = (child as *const u32).read_unaligned();
        let target = ((vtable as *const u32).wrapping_byte_offset(LINK_SLOT as isize))
            .read_unaligned();
        let link: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let answer = link(child, arg, 1, 0);
        if (answer as u8) == 0 {
            return ((this as *const u32).wrapping_byte_offset(CHILD as isize)).read_unaligned();
        }
        let flag_word = (child as *mut u32).wrapping_byte_offset(FLAG as isize);
        flag_word.write_unaligned(flag_word.read_unaligned() | 2);
        0
    }
});
