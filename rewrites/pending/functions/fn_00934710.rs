// original: 0x00934710 slot_detail_ptr
/// 0x00934710: map a slot index to its detail record: read the selector
/// dword at the head of slot `idx`, scale it by the detail stride, and add
/// the detail base.
// shared layout constant
const REC_B_STRIDE: u32 = 0x168;
// shared layout constant
const SCAN_STRIDE: u32 = 0x22c;
// shared layout constant
const SET_RECORDS: usize = 0x144;
export!(thiscall, rw_00934710(this: *const u8, idx: u32) -> u32 {
    unsafe {
        let slots = core::ptr::read_unaligned(this.add(SET_RECORDS) as *const u32);
        let sel = core::ptr::read_unaligned(
            slots.wrapping_add(idx.wrapping_mul(SCAN_STRIDE)) as *const u32,
        );
        let details = core::ptr::read_unaligned(this.add(0x14c) as *const u32);
        sel.wrapping_mul(REC_B_STRIDE).wrapping_add(details)
    }
});
