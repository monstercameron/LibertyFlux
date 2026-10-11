// original: 0x00935BC0 reset_network_record_state

/// Reset network record state when the receiver's byte at `+0x167` is set.
/// The reset path calls the record helper, clears active states through the
/// state-table helper, zeros the 16-bit record count and calls the associated
/// object helper using the pointer stored at the image global `0x01bb5624`
/// plus `0x158`. It then clears the receiver word at `+0x166`; when byte
/// `+0x168` was set, it clears that byte and tail-calls the state transition
/// helper. This is a void-style thiscall routine with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00935bc0(this: u32) -> () {
    unsafe {
        const RESET_GATE: u32 = 0x167;
        const RECORD_COUNT: u32 = 0x148;
        const ASSOCIATED_OBJECT_GLOBAL: u32 = 0x01bb_5624;
        const ASSOCIATED_OBJECT: u32 = 0x158;
        const STATE_FLAGS: u32 = 0x166;
        const TRANSITION_FLAG: u32 = 0x168;

        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        if read_u8(this.wrapping_add(RESET_GATE)) == 0 {
            return;
        }

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
        write_u16(this.wrapping_add(RECORD_COUNT), 0);

        let associated_object = unsafe {
            lf_checker_rt::global::<u32>(ASSOCIATED_OBJECT_GLOBAL).read_unaligned()
        };
        let _ = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            associated_object.wrapping_add(ASSOCIATED_OBJECT)
        );

        let should_transition = read_u8(this.wrapping_add(TRANSITION_FLAG)) != 0;
        write_u16(this.wrapping_add(STATE_FLAGS), 0);
        if should_transition {
            write_u8(this.wrapping_add(TRANSITION_FLAG), 0);
            let _ = lf_checker_rt::callee_thiscall!(4, u32, this);
        }
    }
});
