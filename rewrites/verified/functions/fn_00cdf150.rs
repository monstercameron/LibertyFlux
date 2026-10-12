// original: 0x00CDF150 nm_message_timeout_clear

/// When the task is active, call the ped predicate. A zero result enters the
/// message-send path directly; a nonzero result enters it only when the ready
/// byte is set and the unsigned game timer is strictly beyond the deadline.
/// The method stages a zero-valued NM message context, dispatches it through
/// the ped's behaviour object, then clears the task's deadline and active
/// byte. The stack cookie helper is called last and preserves live registers.
lf_checker_rt::export!(thiscall, rw_00cdf150(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_DEADLINE: u32 = 0x1c;
        const TASK_CLEAR_WORD: u32 = 0x24;
        const TASK_ACTIVE_BYTE: u32 = 0x29;
        const TASK_READY_BYTE: u32 = 0x28;
        const PED_BEHAVIOUR: u32 = 0x7b4;
        const GAME_TIMER_VA: u32 = 0x0117_35b4;
        const COOKIE_VA: u32 = 0x0105_7fb4;
        const NM_BOOL_NAME_VA: u32 = 0x0105_1cc8;
        const NM_SEND_NAME_VA: u32 = 0x0105_216c;
        const CONTEXT_COUNT_OFFSET: usize = 0xcc0 / 4;

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
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        if unsafe { read_u8(this.wrapping_add(TASK_ACTIVE_BYTE)) } != 0 {
            let predicate = lf_checker_rt::callee_thiscall!(1, u32, this, ped) as u8;
            let should_send = predicate == 0
                || (unsafe { read_u8(this.wrapping_add(TASK_READY_BYTE)) } != 0
                    && unsafe {
                        let deadline = read_u32(this.wrapping_add(TASK_DEADLINE));
                        let timer =
                            lf_checker_rt::global::<u32>(GAME_TIMER_VA).read_unaligned();
                        timer > deadline
                    });
            if should_send {
                // This fixed 32-bit context has its counter at byte offset 0xCC0.
                let mut context = [0u32; CONTEXT_COUNT_OFFSET + 1];
                let context_ptr = context.as_mut_ptr() as u32;
                let _context_init =
                    lf_checker_rt::callee_thiscall!(2, u32, context_ptr);
                let bool_name = unsafe {
                    lf_checker_rt::global::<u32>(NM_BOOL_NAME_VA).read_unaligned()
                };
                let _append_bool =
                    lf_checker_rt::callee_thiscall!(3, u32, context_ptr, bool_name, 0);
                let behaviour = unsafe {
                    read_u32(ped.wrapping_add(PED_BEHAVIOUR))
                };
                let send_name = unsafe {
                    lf_checker_rt::global::<u32>(NM_SEND_NAME_VA).read_unaligned()
                };
                let _send_message = lf_checker_rt::callee_thiscall!(
                    4,
                    u32,
                    behaviour,
                    send_name,
                    context_ptr,
                );
                unsafe {
                    write_u32(this.wrapping_add(TASK_CLEAR_WORD), 0);
                    write_u8(this.wrapping_add(TASK_ACTIVE_BYTE), 0);
                }
                let _context_clear =
                    lf_checker_rt::callee_thiscall!(5, u32, context_ptr);
            }
        }

        let cookie = unsafe { lf_checker_rt::global::<u32>(COOKIE_VA).read_unaligned() };
        let _cookie_check = lf_checker_rt::callee_thiscall!(6, u32, cookie);
        0
    }
});
