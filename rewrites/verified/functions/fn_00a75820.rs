// original: 0x00A75820 task_object_build_from_map_data

/// Clears the existing child pointer, looks up task data from the supplied
/// entity, and conditionally constructs and attaches a replacement child.
///
/// The entity argument contains a data pointer at `+0x2c4`; that data's word
/// at `+0x20` is forwarded to the map helper. Two one-word local outputs are
/// passed to the lookup and construction helpers and observed again at later
/// calls. Their addresses are temporary stack addresses, so the checker
/// compares their pointed-to contents through call snapshots instead.
lf_checker_rt::export!(thiscall, rw_00a75820(this: u32, entity: u32) -> u32 {
    unsafe {
        const RELEASE_CHILD: u32 = 4;
        const LOOKUP_DATA: u32 = 1;
        const FIND_TASK: u32 = 2;
        const BUILD_TASK: u32 = 3;
        const ATTACH_TASK: u32 = 5;
        const CHILD_OFFSET: u32 = 0x14;
        const MAP_OFFSET: u32 = 0x2B0;
        const DATA_POINTER_OFFSET: u32 = 0x2C4;

        let child_slot = this.wrapping_add(CHILD_OFFSET);
        let old_child = (child_slot as *const u32).read_unaligned();
        if old_child != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_CHILD, u32, old_child, child_slot);
            (child_slot as *mut u32).write_unaligned(0);
        }

        let data_pointer = ((entity + DATA_POINTER_OFFSET) as *const u32).read_unaligned();
        let map_result = lf_checker_rt::callee_thiscall!(LOOKUP_DATA, u32, entity + MAP_OFFSET);
        if map_result == 0 || data_pointer == 0 {
            return 0;
        }

        let key = ((data_pointer + 0x20) as *const u32).read_unaligned();
        let mut output_primary = 0u32;
        let mut output_secondary = 0u32;
        let found = lf_checker_rt::callee_thiscall!(
            FIND_TASK,
            u32,
            map_result,
            entity,
            key,
            &mut output_secondary as *mut u32 as u32,
            &mut output_primary as *mut u32 as u32,
        );
        if found as u8 == 0 {
            return 0;
        }

        let child = lf_checker_rt::callee_cdecl!(
            BUILD_TASK,
            u32,
            entity,
            &mut output_secondary as *mut u32 as u32,
            &mut output_primary as *mut u32 as u32,
            1u32,
            1u32,
        );
        (child_slot as *mut u32).write_unaligned(child);
        if child != 0 {
            let _ = lf_checker_rt::callee_thiscall!(ATTACH_TASK, u32, child, child_slot);
        }
        0
    }
});
