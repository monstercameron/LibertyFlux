// original: 0x00BC3CB0 CTaskSimpleSay::vf17

/// Advance the speech task's timer state, issue its helper call, and report
/// whether the signed deadline has elapsed. The task object stores an active
/// byte at +0x24, a pending-reset byte at +0x25, a start tick at +0x1c, a
/// duration at +0x20, and a 16-bit speech id at +0x14. The thiscall method
/// pops its one context argument and returns its status in AL.
lf_checker_rt::export!(thiscall, rw_00BC3CB0(this: u32, context: u32) -> u32 {
    const SPEECH_ID: u32 = 0x14;
    const INITIAL_TICK: u32 = 0x18;
    const START_TICK: u32 = 0x1c;
    const DURATION_TICKS: u32 = 0x20;
    const ACTIVE: u32 = 0x24;
    const RESET_PENDING: u32 = 0x25;
    const CURRENT_TICK: u32 = 0x0117_35b4;
    const CALLEE: u32 = 1;
    const ONE_F32: f32 = f32::from_bits(0x3f80_0000);

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        if read_byte(this.wrapping_add(ACTIVE)) == 0 {
            let current_tick = lf_checker_rt::global::<u32>(CURRENT_TICK).read_unaligned();
            let elapsed_ticks = read_word(this.wrapping_add(INITIAL_TICK));
            write_word(this.wrapping_add(START_TICK), current_tick);
            write_word(this.wrapping_add(DURATION_TICKS), elapsed_ticks);
            write_byte(this.wrapping_add(ACTIVE), 1);
        }

        let speech_id = (this.wrapping_add(SPEECH_ID) as *const u16).read_unaligned() as u32;
        let _helper_result = lf_checker_rt::callee_thiscall!(
            CALLEE,
            u32,
            context,
            speech_id,
            0_u32,
            ONE_F32.to_bits(),
            0_u32,
            0_u32,
            0_u32,
        );

        if read_byte(this.wrapping_add(ACTIVE)) == 0 {
            return 0;
        }
        if read_byte(this.wrapping_add(RESET_PENDING)) != 0 {
            let current_tick = lf_checker_rt::global::<u32>(CURRENT_TICK).read_unaligned();
            write_word(this.wrapping_add(START_TICK), current_tick);
            write_byte(this.wrapping_add(RESET_PENDING), 0);
        }

        let deadline = read_word(this.wrapping_add(DURATION_TICKS))
            .wrapping_add(read_word(this.wrapping_add(START_TICK)));
        let current_tick = lf_checker_rt::global::<u32>(CURRENT_TICK).read_unaligned();
        u32::from((deadline as i32) <= (current_tick as i32))
    }
});
