// original: 0x00967420 timing_subsystem_dispatch
/// Release the selected and local player lists, update a valid group slot,
/// then attach the local player to the reference selected by the global index.
///
/// A null local-player context returns after processing the selected list. For
/// a live context, the object linked at `+0x228` supplies a signed group slot
/// at `+0x444`. Nonnegative slots select a `0x5C`-byte global-table entry; a
/// zero group-size answer also assigns the local player as leader. The global
/// current index chooses either a null reference or an indexed reference-table
/// value. The final helper's return is forwarded.
///
/// Direct callees are intercepted and scripted. The proof compares each call,
/// its arguments and the scripted return; it does not execute the helpers'
/// internal list, group, or reference-table operations. A live context always
/// links a readable group object; the original faults if that nested pointer is
/// null. The global table entries used by the function are separately seeded.
lf_checker_rt::export!(cdecl, rw_00967420() -> u32 {
    unsafe {
        const GROUP_SLOT: u32 = 0x228;
        const GROUP_INDEX: u32 = 0x3D4;
        const GROUP_TABLE_VA: u32 = 0x0166_6C88;
        const CURRENT_INDEX_VA: u32 = 0x0103_6F14;
        const REFERENCE_TABLE_VA: u32 = 0x011A_8808;
        const GROUP_ENTRY_STRIDE: u32 = 0x5C;

        let local_player = lf_checker_rt::callee_cdecl!(1, u32,);
        let selected_list: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0);
        if selected_list != 0 {
            let _ = lf_checker_rt::callee_thiscall!(3, u32, selected_list);
        }

        if local_player == 0 {
            return 0;
        }
        let _ = lf_checker_rt::callee_thiscall!(3, u32, local_player);

        let group_object = (local_player.wrapping_add(GROUP_SLOT) as *const u32)
            .read_unaligned();
        let group_index_address = group_object
            .wrapping_add(0x70)
            .wrapping_add(GROUP_INDEX);
        let group_index = (group_index_address as *const i32).read_unaligned();

        if group_index >= 0 {
            let row_offset = (group_index as u32).wrapping_mul(GROUP_ENTRY_STRIDE);
            let group_row = (lf_checker_rt::global::<u8>(GROUP_TABLE_VA) as u32)
                .wrapping_add(row_offset);
            let group_size: u32 = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                group_row.wrapping_add(8),
            );
            if group_size == 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    5,
                    u32,
                    group_row.wrapping_add(8),
                    local_player,
                );
            }
            let _ = lf_checker_rt::callee_thiscall!(6, u32, group_row);
        }

        let current_index = *lf_checker_rt::global::<i32>(CURRENT_INDEX_VA);
        let reference = if current_index == -1 {
            0
        } else {
            let reference_slot = REFERENCE_TABLE_VA
                .wrapping_add((current_index as u32).wrapping_mul(4));
            *lf_checker_rt::global::<u32>(reference_slot)
        };
        lf_checker_rt::callee_thiscall!(7, u32, reference, local_player)
    }
});
