// original: 0x00B7F230 update_drive_model_request

/// Looks up a drive-by seat row for the supplied pedestrian, stores either its
/// selected row index or the invalid sentinel, and asks the task registry to
/// apply that index. The lookup helper writes one result word through a pointer
/// to a local stack slot. The security-cookie check receives the cookie value
/// loaded from the image's writable data section. Thiscall pops the pedestrian
/// pointer; the routine's return register is incidental.
lf_checker_rt::export!(thiscall, rw_b7f230(this: u32, ped: u32) -> () {
    unsafe {
        const LOOKUP_SEAT_ROW: u32 = 1;
        const CHANGE_ACTIVE_TASK: u32 = 2;
        const CHECK_SECURITY_COOKIE: u32 = 3;
        const SEAT_HINT: u32 = 0x2b0;
        const ACTIVE_TASK_FIELD: u32 = 0x34;
        const ACTIVE_TASK_INDEX: u32 = 0x40;
        const ACTIVE_TASK_STATE: u32 = 0x44;
        const INVALID_TASK_INDEX: u32 = u32::MAX;
        const SECURITY_COOKIE: u32 = 0x01057fb4;

        let mut seat_result = core::mem::MaybeUninit::<u32>::uninit();
        let seat_result_ptr = seat_result.as_mut_ptr() as u32;
        let requested_seat = (ped.wrapping_add(SEAT_HINT) as *const u32).read_unaligned();
        let found = lf_checker_rt::callee_cdecl!(
            LOOKUP_SEAT_ROW,
            u8,
            ped,
            seat_result_ptr,
            requested_seat,
        );
        let selected_index = if found != 0 {
            seat_result.assume_init()
        } else {
            INVALID_TASK_INDEX
        };
        let active_index = if found != 0 && selected_index != INVALID_TASK_INDEX {
            selected_index
        } else {
            INVALID_TASK_INDEX
        };
        (this.wrapping_add(ACTIVE_TASK_INDEX) as *mut u32).write_unaligned(active_index);

        let registry_state = lf_checker_rt::callee_thiscall!(
            CHANGE_ACTIVE_TASK,
            u8,
            this.wrapping_add(ACTIVE_TASK_FIELD),
            active_index,
        );
        (this.wrapping_add(ACTIVE_TASK_STATE) as *mut u8).write(registry_state);

        let cookie = (lf_checker_rt::global::<u32>(SECURITY_COOKIE) as *const u32).read_unaligned();
        let _ = lf_checker_rt::callee_thiscall!(CHECK_SECURITY_COOKIE, u32, cookie);
    }
});
