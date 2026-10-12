// original: 0x005C6170 ped_task_hud_flags_update

/// Sets the HUD marker byte on five task objects. The global gates select one
/// byte value: one when every active-task predicate holds, otherwise zero.
/// Each global index selects a pointer from the task-object table, and the
/// selected object's byte at offset 0x12 is updated in the original order.
lf_checker_rt::export!(cdecl, rw_005c6170() -> u32 {
    unsafe {
        const HUD_ENABLED: u32 = 0x011e61c6;
        const TASK_LIST_READY: u32 = 0x01160c9c;
        const TASK_MODE: u32 = 0x011f7060;
        const TASK_STATE: u32 = 0x012088b4;
        const TASK_KIND: u32 = 0x01037720;
        const OBJECT_TABLE: u32 = 0x0118e7f8;
        const MARKER_OFFSET: u32 = 0x12;
        const EXPECTED_MODE: u32 = 1;
        const BLOCKED_STATE: u32 = u32::MAX;
        const EXCLUDED_KIND: u32 = 0x12;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(address).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(address).read() }
        }
        #[inline(always)]
        unsafe fn write_byte(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let marker_value = u8::from(
            read_byte(HUD_ENABLED) != 0
                && read_word(TASK_LIST_READY) != 0
                && read_word(TASK_MODE) != EXPECTED_MODE
                && read_word(TASK_STATE) == BLOCKED_STATE
                && read_word(TASK_KIND) != EXCLUDED_KIND,
        );
        const TASK_INDEX_GLOBALS: [u32; 5] = [
            0x0118edc0,
            0x0118eddc,
            0x0118eda4,
            0x0118edf8,
            0x0118ee14,
        ];
        let mut last_object = 0;
        for index_global in TASK_INDEX_GLOBALS {
            let index = read_word(index_global);
            let object = read_word(OBJECT_TABLE.wrapping_add(index.wrapping_mul(4)));
            write_byte(object.wrapping_add(MARKER_OFFSET), marker_value);
            last_object = object;
        }
        last_object
    }
});
