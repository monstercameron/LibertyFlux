// original: 0x00DF6770 UIFileViewer::delete_selected_montage_clip

/// Delete the currently selected clip when its viewer and record are ready.
///
/// Resolve the candidate child, verify its state, follow the child record to
/// its entry index, and pass that index to the gallery-record update method.
/// A successful update emits the success notification, stores the selected
/// record number at `+0x308`, sets the activity byte at `+0x314`, and selects
/// input mode `0x24`. A final gallery veto emits the failure notification,
/// sets the activity byte, selects mode `0x29`, and returns false. Missing or
/// inactive candidate children return false without those side effects. The
/// return value is a Boolean in AL.
lf_checker_rt::export!(thiscall, rw_df6770(this: u32) -> u32 {
    unsafe {
        const ACTIVE_ENTRY_FIELD: u32 = 0x1E0;
        const SELECTED_RECORD: u32 = 0x308;
        const ACTIVITY_FLAG: u32 = 0x314;
        const NOTIFICATION_MANAGER: u32 = 0x1176888;

        let candidate = lf_checker_rt::callee_thiscall!(1, u32, this);
        if candidate == 0 { return 0; }

        let candidate_ready = lf_checker_rt::callee_thiscall!(2, u32, candidate);
        if candidate_ready == 0 { return 0; }

        let active_child = ((candidate.wrapping_add(ACTIVE_ENTRY_FIELD)) as *const u32)
            .read_unaligned();
        let record = lf_checker_rt::callee_thiscall!(4, u32, active_child);
        let entry_list = lf_checker_rt::callee_thiscall!(5, u32, record);
        let entry_index = lf_checker_rt::callee_thiscall!(6, u32, entry_list);
        let should_delete = lf_checker_rt::callee_thiscall!(7, u32, this, entry_index);

        if should_delete as u8 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                8, u32,
                lf_checker_rt::relocated(NOTIFICATION_MANAGER),
                lf_checker_rt::relocated(0xF01BF4)
            );
            let selected_record = lf_checker_rt::callee_thiscall!(3, u32, candidate);
            ((this.wrapping_add(SELECTED_RECORD)) as *mut u32)
                .write_unaligned(selected_record);
            ((this.wrapping_add(ACTIVITY_FLAG)) as *mut u8).write(1);
            let _: u32 = lf_checker_rt::callee_cdecl!(9, u32, 0x24u32);
            return 1;
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(
            8, u32,
            lf_checker_rt::relocated(NOTIFICATION_MANAGER),
            lf_checker_rt::relocated(0xF01C1C)
        );
        ((this.wrapping_add(ACTIVITY_FLAG)) as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_cdecl!(9, u32, 0x29u32);
        0
    }
});
