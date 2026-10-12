// original: 0x0087B5B0 clear_child_state

/// Clear a nested child and the base state twice, switching the vtable each time.
///
/// `this` is a node-frame object with an optional child at `+0x1c`, a child
/// destructor flag at `+0x20`, and an optional owned state pointer at `+0x08`.
/// The method installs the first relocated vtable, conditionally invokes the
/// child's slot-zero destructor with delete flag one, clears the child pointer,
/// and calls the base teardown helper. It clears `+0x0c` and `+0x10`, clears
/// `+0x08` when nonzero, installs the second relocated vtable and repeats the
/// base teardown and state clearing. It returns the second helper's result.
///
/// Original: thiscall with no stack arguments. The child call is intercepted
/// through a planted object function table; both direct helpers are patched.
lf_checker_rt::export!(thiscall, rw_0087b5b0(this: u32) -> u32 {
    const CHILD_OFFSET: u32 = 0x1C;
    const DELETE_CHILD_OFFSET: u32 = 0x20;
    const OWNED_STATE_OFFSET: u32 = 0x08;
    const STATE_COUNT_OFFSET: u32 = 0x0C;
    const STATE_POSITION_OFFSET: u32 = 0x10;
    const FIRST_VTABLE_FILE_VA: u32 = 0x00FE_83D4;
    const SECOND_VTABLE_FILE_VA: u32 = 0x00FE_7F28;
    const CHILD_DESTRUCTOR: u32 = 1;
    const BASE_TEARDOWN: u32 = 2;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let delete_child = (this.wrapping_add(DELETE_CHILD_OFFSET) as *const u8).read() != 0;
        write_u32(this, lf_checker_rt::relocated(FIRST_VTABLE_FILE_VA));

        let child = read_u32(this.wrapping_add(CHILD_OFFSET));
        if delete_child && child != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CHILD_DESTRUCTOR, u32, child, 1u32);
        }

        write_u32(this.wrapping_add(CHILD_OFFSET), 0);
        let _ = lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this);
        let owned_state = read_u32(this.wrapping_add(OWNED_STATE_OFFSET));
        write_u32(this.wrapping_add(STATE_COUNT_OFFSET), 0);
        write_u32(this.wrapping_add(STATE_POSITION_OFFSET), 0);
        if owned_state != 0 {
            write_u32(this.wrapping_add(OWNED_STATE_OFFSET), 0);
        }

        write_u32(this, lf_checker_rt::relocated(SECOND_VTABLE_FILE_VA));
        let final_result = lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this);
        let owned_state = read_u32(this.wrapping_add(OWNED_STATE_OFFSET));
        write_u32(this.wrapping_add(STATE_COUNT_OFFSET), 0);
        write_u32(this.wrapping_add(STATE_POSITION_OFFSET), 0);
        if owned_state != 0 {
            write_u32(this.wrapping_add(OWNED_STATE_OFFSET), 0);
        }
        final_result
    }
});
