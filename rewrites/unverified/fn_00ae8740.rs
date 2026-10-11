// original: 0x00AE8740 reset_timer_allocations

/// Refreshes a timer manager, delegates the active timer work, then clears
/// its four per-frame fields at `+0x24`, `+0x28`, `+0x2c` and `+0x30`. It
/// context also supplies the category value at `+0x938`. A global byte
/// selects the extended path, which makes three extra subsystem calls before
/// the shared manager callback. The routine sets and then clears two global
/// status bytes around that work.
///
/// Calling convention: cdecl with one pointer to a fixed 32-bit manager
/// layout. The four per-frame values and the category are passed to callbacks
/// in that order as specified by each callback's stack arguments.
lf_checker_rt::export!(cdecl, rw_00ae8740(manager: u32) -> () {
    unsafe {
        const MANAGER_FRAME_FIRST: u32 = 0x24;
        const MANAGER_FRAME_SECOND: u32 = 0x28;
        const MANAGER_FRAME_THIRD: u32 = 0x2c;
        const MANAGER_FRAME_FOURTH: u32 = 0x30;
        const MANAGER_CATEGORY: u32 = 0x938;
        const LAST_TIMER_VA: u32 = 0x0159_3bc4;
        const CURRENT_TIMER_VA: u32 = 0x0159_3bc8;
        const EXTENDED_PATH_VA: u32 = 0x0103_f718;
        const INITIAL_STATUS_VA: u32 = 0x0103_3108;
        const FRAME_STATUS_VA: u32 = 0x015a_e64d;
        const TIMER_SAMPLE_CALLEE: u32 = 1;
        const EXTENDED_SAMPLE_CALLEE: u32 = 2;
        const SUBSYSTEM_CALLEE_A: u32 = 3;
        const SUBSYSTEM_CALLEE_B: u32 = 4;
        const SUBSYSTEM_CALLEE_C: u32 = 5;
        const MANAGER_CALLEE: u32 = 6;
        const SERVICE_CALLEE: u32 = 7;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        write_u8(lf_checker_rt::global::<u8>(INITIAL_STATUS_VA) as u32, 1);
        let category = read_u32(manager.wrapping_add(MANAGER_CATEGORY));
        let timer_sample = lf_checker_rt::callee_cdecl!(TIMER_SAMPLE_CALLEE, u32,);
        let current_timer = read_u32(lf_checker_rt::global::<u32>(CURRENT_TIMER_VA) as u32);
        let _aligned_elapsed = current_timer.wrapping_sub(timer_sample) & !7;
        let extended = read_u8(lf_checker_rt::global::<u8>(EXTENDED_PATH_VA) as u32) != 0;
        write_u8(lf_checker_rt::global::<u8>(FRAME_STATUS_VA) as u32, 1);

        let (sample, older_value) = if extended {
            let sample = lf_checker_rt::callee_cdecl!(EXTENDED_SAMPLE_CALLEE, u32,);
            let older_value = read_u32(lf_checker_rt::global::<u32>(LAST_TIMER_VA) as u32);
            lf_checker_rt::callee_cdecl!(SUBSYSTEM_CALLEE_A, u32, category, sample, older_value, manager);
            lf_checker_rt::callee_cdecl!(SUBSYSTEM_CALLEE_B, u32, category, sample, older_value, manager);
            lf_checker_rt::callee_cdecl!(SUBSYSTEM_CALLEE_C, u32, category, sample, older_value, manager);
            (sample, older_value)
        } else {
            (
                read_u32(manager.wrapping_add(MANAGER_FRAME_FIRST)),
                read_u32(manager.wrapping_add(MANAGER_FRAME_SECOND)),
            )
        };

        lf_checker_rt::callee_cdecl!(MANAGER_CALLEE, u32, category, sample, older_value, manager);
        let service = lf_checker_rt::relocated(0x0117_3750);
        lf_checker_rt::callee_thiscall!(SERVICE_CALLEE, u32, service);

        write_u8(lf_checker_rt::global::<u8>(FRAME_STATUS_VA) as u32, 0);
        write_u32(manager.wrapping_add(MANAGER_FRAME_FIRST), 0);
        write_u32(manager.wrapping_add(MANAGER_FRAME_THIRD), 0);
        write_u32(manager.wrapping_add(MANAGER_FRAME_SECOND), 0);
        write_u32(manager.wrapping_add(MANAGER_FRAME_FOURTH), 0);
    }
});
