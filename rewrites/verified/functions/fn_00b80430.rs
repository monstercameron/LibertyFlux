// original: 0x00B80430 ped_task_manager_drive_task_update

/// Lazily attach the update source stored at `game_state + 0x0b30` to the
/// embedded task pointer at `this + 0x40`, notifying the helper with the field's
/// address when the source is non-null. Then return true when the embedded
/// pointer is null or when the completed state and cached byte are both zero.
/// Otherwise copy the nonzero state to byte `+0x3e`, write `field + 0x48` plus
/// the global timer to `+0x24`, clear bit two at `+0x3f`, and reset `+0x44`.
lf_checker_rt::export!(thiscall, rw_b80430(this: u32, game_state: u32) -> u32 {
    unsafe {
        const EMBEDDED_SOURCE: u32 = 0x40;
        const COMPLETED_STATE: u32 = 0x44;
        const LAST_TIMER: u32 = 0x48;
        const UPDATE_SOURCE_OFFSET: u32 = 0x0b30;
        const UPDATE_SOURCE_HELPER: u32 = 1;
        const TIMER_GLOBAL: u32 = 0x011735b4;

        let source_field = this.wrapping_add(EMBEDDED_SOURCE);
        let mut source = (source_field as *const u32).read_unaligned();
        if source == 0 {
            source = ((game_state.wrapping_add(UPDATE_SOURCE_OFFSET)) as *const u32).read_unaligned();
            (source_field as *mut u32).write_unaligned(source);
            if source != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    UPDATE_SOURCE_HELPER,
                    u32,
                    source,
                    source_field
                );
            }
        }

        let state = ((this.wrapping_add(COMPLETED_STATE)) as *const u32).read_unaligned();
        let cached = ((this.wrapping_add(0x3e)) as *const u8).read();
        if state == 0 && cached == 0 {
            return 1;
        }
        if source == 0 {
            return 1;
        }
        if state != 0 {
            ((this.wrapping_add(0x3e)) as *mut u8).write(state as u8);
            let last_timer = ((this.wrapping_add(LAST_TIMER)) as *const u32).read_unaligned();
            let current_timer = (lf_checker_rt::relocated(TIMER_GLOBAL) as *const u32).read_unaligned();
            ((this.wrapping_add(0x3f)) as *mut u8).write(
                ((this.wrapping_add(0x3f)) as *const u8).read() & !0x04,
            );
            ((this.wrapping_add(0x24)) as *mut u32).write_unaligned(last_timer.wrapping_add(current_timer));
            ((this.wrapping_add(COMPLETED_STATE)) as *mut u32).write_unaligned(0);
        }
        0
    }
});
