// original: 0x00E6ED40 network_destroy_record16_arrays

/// Walks four eight-byte array records in reverse order and destroys each
/// record whose 16-bit count is nonzero. The count and array pointer are the
/// two stack arguments passed to the stdcall destroy routine; its EAX result
/// is the function's final EAX whenever the last record is active. When the
/// last record is empty, EAX is zero. The table lives in writable image data.
lf_checker_rt::export!(cdecl, rw_00e6ed40() -> u32 {
    const TABLE_BASE_VA: u32 = 0x019D_2F84;
    const RECORD_COUNT: u32 = 4;
    const RECORD_STRIDE: u32 = 8;
    const ARRAY_POINTER_OFFSET: u32 = 0;
    const ITEM_COUNT_OFFSET: u32 = 6;
    const DESTROY_ARRAY_CALLEE: u32 = 1;

    // SAFETY: the table locations are the data operands this function reads;
    // the contract supplies their values and intercepts the destroy call.
    unsafe {
        let mut result = 0u32;
        for index in (0..RECORD_COUNT).rev() {
            let record_va = TABLE_BASE_VA + index * RECORD_STRIDE;
            let item_count = lf_checker_rt::global::<u16>(record_va + ITEM_COUNT_OFFSET)
                .read_unaligned() as u32;
            result = if item_count == 0 {
                0
            } else {
                let array = lf_checker_rt::global::<u32>(record_va + ARRAY_POINTER_OFFSET)
                    .read_unaligned();
                lf_checker_rt::callee_stdcall!(DESTROY_ARRAY_CALLEE, u32, array, item_count)
            };
        }
        result
    }
});
