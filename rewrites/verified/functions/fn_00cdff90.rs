// original: 0x00CDFF90 CTaskSimpleNMBrace::vf28

/// Update the Brace task for one ped. The task stores its selector at `+0x50`
/// and its unsigned deadline word at `+0x24`; the ped's enable byte is at
/// `+0x219`. The dispatcher always receives the ped, mode 1, and a selector
/// initially chosen as 8 or 9 from the task byte at `+0x50`. When the ped is
/// enabled and the global timer is strictly beyond the wrapping deadline, bit
/// zero is set in that selector. The dispatcher's result is returned.
lf_checker_rt::export!(thiscall, rw_00cdff90(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_SELECTOR: u32 = 0x50;
        const TASK_DEADLINE: u32 = 0x24;
        const PED_ENABLED: u32 = 0x219;
        const GLOBAL_TIMER: u32 = 0x0117_35b4;
        const DEADLINE_DELTA: u32 = 5_000;
        const DISPATCH_MODE: u32 = 1;
        const SELECTOR_DISABLED: u32 = 8;
        const SELECTOR_ENABLED: u32 = 9;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let mut selector = if unsafe { read_u8(this.wrapping_add(TASK_SELECTOR)) } != 0 {
            SELECTOR_ENABLED
        } else {
            SELECTOR_DISABLED
        };
        if unsafe { read_u8(ped.wrapping_add(PED_ENABLED)) } != 0 {
            let deadline = unsafe { read_u32(this.wrapping_add(TASK_DEADLINE)) }
                .wrapping_add(DEADLINE_DELTA);
            let timer = unsafe {
                lf_checker_rt::global::<u32>(GLOBAL_TIMER).read_unaligned()
            };
            if timer > deadline {
                selector |= 1;
            }
        }
        lf_checker_rt::callee_thiscall!(
            1,
            u32,
            this,
            ped,
            DISPATCH_MODE,
            selector,
        )
    }
});
