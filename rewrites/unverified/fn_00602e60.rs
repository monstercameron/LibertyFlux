// original: 0x00602E60 rage::rmPtfxManager::cull_and_draw_instances

/// Seed the resource-state words from the four read-only source values and set
/// the single sentinel global. Release the pending list, poll until its low
/// byte is zero, and inspect each returned resource. When the resource is
/// ready and its indexed item is active, call the callback with the incoming
/// XMM1 bits among its five stack words. Finally clear the manager's pending
/// field. The release, poll, query, and callback callees are explicit stand-ins
/// in the proof contract.

const MANAGER_PENDING_FIELD: u32 = 0x4BC;
const RESOURCE_STATE_GLOBAL: u32 = 0x0110_ED00;
const RESOURCE_SENTINEL_GLOBAL: u32 = 0x0110_ED10;
const RESOURCE_SOURCE_CONSTANT: u32 = 0x00FE_8FB0;
const RESOURCE_RECORDS_FIELD: u32 = 0x8C;
const RESOURCE_RECORD_STRIDE: u32 = 0x120;
const RESOURCE_CALLBACK_FIELD: u32 = 0xE0;
const RESOURCE_META_FIELD: u32 = 0x88;
const RESOURCE_ITEM_ARRAY_FIELD: u32 = 0x130;
const RESOURCE_ITEM_ACTIVE_FIELD: u32 = 0x14;
const RESOURCE_ITEM_ARG_A: u32 = 0x4C;
const RESOURCE_ITEM_ARG_B: u32 = 0x0C;
const CALLEE_RELEASE_PENDING: u32 = 1;
const CALLEE_POLL_RESOURCE: u32 = 2;
const CALLEE_QUERY_RESOURCE: u32 = 3;
const CALLEE_RESOURCE_CALLBACK: u32 = 4;

#[inline(always)]
fn read_u32(base: u32, offset: u32) -> u32 {
    unsafe { core::ptr::read_volatile(base.wrapping_add(offset) as *const u32) }
}

#[inline(always)]
fn write_u32(base: u32, offset: u32, value: u32) {
    unsafe { core::ptr::write_volatile(base.wrapping_add(offset) as *mut u32, value) }
}

fn process_manager_resources(manager: u32, leave_pending: bool) {
    let source_constants = lf_checker_rt::relocated(RESOURCE_SOURCE_CONSTANT);
    let resource_state = lf_checker_rt::relocated(RESOURCE_STATE_GLOBAL);
    let resource_sentinels = lf_checker_rt::relocated(RESOURCE_SENTINEL_GLOBAL);

    let source_words = [
        read_u32(source_constants, 0),
        read_u32(source_constants, 4),
        read_u32(source_constants, 8),
        read_u32(source_constants, 12),
    ];
    for (index, word) in source_words.into_iter().enumerate() {
        write_u32(resource_state, (index as u32).wrapping_mul(4), word);
    }
    write_u32(resource_sentinels, 0, u32::MAX);

    let pending_pointer = manager.wrapping_add(MANAGER_PENDING_FIELD);
    let _ = lf_checker_rt::callee_cdecl!(CALLEE_RELEASE_PENDING, u32, pending_pointer);

    loop {
        let poll_result = lf_checker_rt::callee_cdecl!(CALLEE_POLL_RESOURCE, u32,);
        if poll_result as u8 == 0 {
            break;
        }

        let resource_owner = read_u32(resource_state, 4);
        let resource_index = read_u32(resource_state, 8);
        let records = read_u32(resource_owner, RESOURCE_RECORDS_FIELD);
        let record_offset = resource_index.wrapping_mul(RESOURCE_RECORD_STRIDE);
        let candidate = records.wrapping_add(record_offset);
        let record = if read_u32(candidate, RESOURCE_CALLBACK_FIELD) != 0 {
            candidate
        } else {
            0
        };

        if record != 0 && read_u32(record, RESOURCE_CALLBACK_FIELD) != 0 {
            let resource_view = resource_owner.wrapping_add(0x10);
            let query_result = lf_checker_rt::callee_thiscall!(
                CALLEE_QUERY_RESOURCE,
                u32,
                resource_view
            );
            if query_result as u8 == 0 {
                let metadata = read_u32(resource_owner, RESOURCE_META_FIELD);
                let item_array = read_u32(metadata, RESOURCE_ITEM_ARRAY_FIELD);
                let item = read_u32(item_array, resource_index.wrapping_mul(4));
                if item != 0 && read_u32(item, RESOURCE_ITEM_ACTIVE_FIELD) == 0 {
                    let callback_object = read_u32(record, RESOURCE_CALLBACK_FIELD);
                    let blend_bits = lf_checker_rt::xmm_word(1, 0);
                    let _ = lf_checker_rt::callee_thiscall!(
                        CALLEE_RESOURCE_CALLBACK,
                        u32,
                        callback_object,
                        blend_bits,
                        resource_view,
                        record,
                        read_u32(item, RESOURCE_ITEM_ARG_A),
                        read_u32(item, RESOURCE_ITEM_ARG_B)
                    );
                }
            }
        }
    }

    if !leave_pending {
        write_u32(manager, MANAGER_PENDING_FIELD, 0);
    }
}

lf_checker_rt::export!(thiscall, rw_00602e60(manager: u32) -> () {
    process_manager_resources(manager, false)
});