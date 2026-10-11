// original: 0x0095C920 append_task_entries
/// Initialize an object's resource id and append qualifying list entries.
///
/// The cdecl function takes an object pointer. It calls the resource-id helper
/// when the word at offset 0x64 is 0xffffffff, then reads the signed entry
/// count at 0xf8c. For a positive count it visits addresses beginning at the
/// list pointer at 0xf88 with a 0x34-byte stride. Each non-null item address is
/// passed with the object and signed index to a thiscall predicate. Only a
/// predicate whose low EAX byte is exactly 1 advances the shared output cursor
/// by eight bytes. The function has no semantic return value.
lf_checker_rt::export!(cdecl, rw_0095c920(object: u32) -> () {
    const OBJECT_RESOURCE_ID: u32 = 0x64;
    const UNASSIGNED_RESOURCE_ID: u32 = u32::MAX;
    const OBJECT_LIST_BASE: u32 = 0xf88;
    const OBJECT_LIST_COUNT: u32 = 0xf8c;
    const LIST_ITEM_STRIDE: u32 = 0x34;
    const OUTPUT_OFFSET_VA: u32 = 0x0120F290;
    const OUTPUT_BASE_VA: u32 = 0x0120F294;
    const RESOLVE_RESOURCE_ID: u32 = 1;
    const CHECK_TASK_ENTRY: u32 = 2;
    const OUTPUT_BYTES: u32 = 8;

    unsafe {
        let resource_id = (object.wrapping_add(OBJECT_RESOURCE_ID) as *const u32)
            .read_unaligned();
        if resource_id == UNASSIGNED_RESOURCE_ID {
            let resolved_id = lf_checker_rt::callee_cdecl!(
                RESOLVE_RESOURCE_ID,
                u32,
                object
            );
            (object.wrapping_add(OBJECT_RESOURCE_ID) as *mut u32)
                .write_unaligned(resolved_id);
        }

        let count = (object.wrapping_add(OBJECT_LIST_COUNT) as *const u32)
            .read_unaligned() as i32;
        if count > 0 {
            let list_base = (object.wrapping_add(OBJECT_LIST_BASE) as *const u32)
                .read_unaligned();
            let mut index = 0i32;
            let mut item_offset = 0u32;
            while index < count {
                let output_offset = lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA)
                    .read_unaligned();
                let output_base = lf_checker_rt::global::<u32>(OUTPUT_BASE_VA)
                    .read_unaligned();
                let output_cursor = output_base.wrapping_add(output_offset);
                let item_address = list_base.wrapping_add(item_offset);
                if item_address != 0 {
                    let answer = lf_checker_rt::callee_thiscall!(
                        CHECK_TASK_ENTRY,
                        u32,
                        output_cursor,
                        object,
                        index as u32
                    );
                    if answer as u8 == 1 {
                        lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA)
                            .write_unaligned(output_offset.wrapping_add(OUTPUT_BYTES));
                    }
                }
                index = index.wrapping_add(1);
                item_offset = item_offset.wrapping_add(LIST_ITEM_STRIDE);
            }
        }
    }
});
