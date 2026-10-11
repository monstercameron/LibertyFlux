// original: 0x009C07F0 ped_task_record_matches
/// Returns one when `index` is in the unsigned range 0 through 31 and the
/// record's pointer field at byte offset 0x1c equals `record_ptr`; otherwise
/// returns zero. The record table has a 0xd0-byte stride and is accessed
/// through relocated image data.
lf_checker_rt::export!(cdecl, rw_009c07f0(index: u32, record_ptr: u32) -> u8 {
    const MAX_RECORD_INDEX: u32 = 31;
    const RECORD_TABLE_BASE: u32 = 0x0129_2450;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_POINTER_OFFSET: u32 = 0x1C;

    if index > MAX_RECORD_INDEX {
        return 0;
    }
    let pointer_address = RECORD_TABLE_BASE
        .wrapping_add(index.wrapping_mul(RECORD_STRIDE))
        .wrapping_add(RECORD_POINTER_OFFSET);
    let stored_pointer = unsafe { lf_checker_rt::global::<u32>(pointer_address).read() };
    u8::from(stored_pointer == record_ptr)
});
