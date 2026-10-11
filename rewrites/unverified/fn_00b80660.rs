// original: 0x00B80660 ped_task_manager_update_target_setting

/// Update the target setting when the task has a handle and the supplied game
/// state contains a player. Continue only when the signed wanted-level value
/// is greater than zero and the settings helper does not return `u32::MAX`.
/// Store the selected setting at `+0x20`, pass it to the embedded settings
/// method, and write that method's low byte to `+0x24`. All failed conditions
/// store `u32::MAX`, call the method with that sentinel, and clear `+0x24`.
lf_checker_rt::export!(thiscall, rw_b80660(this: u32, game_state: u32) -> u32 {
    unsafe {
        const TASK_HANDLE: u32 = 0x28;
        const PLAYER_OFFSET: u32 = 0x228;
        const WANTED_LEVEL_VIEW: u32 = 0x70;
        const SETTING_FIELD: u32 = 0x20;
        const SETTING_RESULT: u32 = 0x24;
        const WANTED_LEVEL: u32 = 1;
        const LOOKUP_SETTING: u32 = 2;
        const APPLY_SETTING: u32 = 3;

        let fail = || {
            unsafe {
                ((this + SETTING_FIELD) as *mut u32).write_unaligned(u32::MAX);
                let result: u32 = lf_checker_rt::callee_thiscall!(
                    APPLY_SETTING,
                    u32,
                    this + 0x14,
                    u32::MAX
                );
                ((this + SETTING_RESULT) as *mut u8).write(0);
                result
            }
        };

        let task_handle = ((this + TASK_HANDLE) as *const u32).read_unaligned();
        if task_handle == 0 {
            return fail();
        }
        let player = ((game_state + PLAYER_OFFSET) as *const u32).read_unaligned();
        if player == 0 {
            return fail();
        }
        let wanted_level: u32 = lf_checker_rt::callee_thiscall!(
            WANTED_LEVEL,
            u32,
            player.wrapping_add(WANTED_LEVEL_VIEW)
        );
        if (wanted_level as i32) <= 0 {
            return fail();
        }
        let setting: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_SETTING, u32, task_handle);
        if setting == u32::MAX {
            return fail();
        }
        ((this + SETTING_FIELD) as *mut u32).write_unaligned(setting);
        let result: u32 = lf_checker_rt::callee_thiscall!(
            APPLY_SETTING,
            u32,
            this + 0x14,
            setting
        );
        ((this + SETTING_RESULT) as *mut u8).write(result as u8);
        result
    }
});
