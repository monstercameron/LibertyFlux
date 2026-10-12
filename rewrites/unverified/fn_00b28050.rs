// original: 0x00B28050 NativeImpl_GET_TOTAL_DURATION_OF_CAR_RECORDING
/// Find a recording identifier in the initialized recording table and return
/// its zero-based slot. The table count is signed: a zero or negative count
/// produces no match. Each entry is sixteen bytes, with its ID in the first
/// word. This cdecl helper receives one identifier and returns an integer slot.
const RECORDING_COUNT_VA: u32 = 0x0165_fb90;
const RECORDING_IDS_VA: u32 = 0x0165_7a10;
const RECORDING_ID_STRIDE: u32 = 0x10;

lf_checker_rt::export!(cdecl, rw_b28050(recording_id: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 { (address as *const u32).read_unaligned() }
        let count_address = lf_checker_rt::relocated(RECORDING_COUNT_VA);
        let ids_address = lf_checker_rt::relocated(RECORDING_IDS_VA);
        let count = read_word(count_address) as i32;
        if count <= 0 { return 0; }
        for slot_index in 0..(count as u32) {
            let slot = ids_address.wrapping_add(slot_index * RECORDING_ID_STRIDE);
            if read_word(slot) == recording_id { return slot_index; }
        }
        0
    }
});
