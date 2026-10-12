// original: 0x0089A8A0 rage::audEnvelopeSound::drive_state
/// Advance an envelope sound node using its current state and update time.
///
/// The thiscall takes a node pointer and an unsigned update time. It returns
/// true immediately when the node is already active and not pending. Otherwise
/// it computes an elapsed time only when the signed start field is nonnegative,
/// promotes a pending zero state to state one, and advances state two through
/// the resolver. State one forwards elapsed time and update time to the child
/// stage. The resolver and stage calls are intercepted by the proof contract.
/// The boolean result is observed in AL; upper EAX bytes are not part of this
/// method's result contract.
lf_checker_rt::export!(thiscall, rw_0089a8a0(
    this: u32,
    update_time: u32,
) -> u32 {
    unsafe {
        const ACTIVE_FLAGS: u32 = 0x38;
        const START_TIME: u32 = 0x84;
        const STATE: u32 = 0xf5;
        const PENDING_FLAGS: u32 = 0xf8;
        const ACTIVE_BIT: u8 = 0x10;
        const PENDING_BIT: u8 = 2;
        const RESOLVING: u8 = 2;
        const RUNNING: u8 = 1;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let flags = read_u8(this.wrapping_add(ACTIVE_FLAGS));
        let pending = read_u8(this.wrapping_add(PENDING_FLAGS));
        if flags & ACTIVE_BIT != 0 && pending & PENDING_BIT == 0 {
            return 1;
        }

        let signed_start = read_u32(this.wrapping_add(START_TIME)) as i32;
        let elapsed = if signed_start >= 0 {
            update_time.wrapping_sub(signed_start as u32)
        } else {
            0
        };

        let state = read_u8(this.wrapping_add(STATE));
        if state == 0 {
            if pending & PENDING_BIT != 0 {
                write_u8(this.wrapping_add(STATE), RUNNING);
            }
        } else if state == RESOLVING {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        }

        let state_after_resolver = read_u8(this.wrapping_add(STATE));
        if state_after_resolver == RUNNING {
            return lf_checker_rt::callee_thiscall!(2, u32, this, elapsed, update_time);
        }
        1
    }
});
