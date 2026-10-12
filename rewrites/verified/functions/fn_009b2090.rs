// original: 0x009B2090 audPoliceScanner::start_slot_sound

/// Starts every sound registered in the selected slot. It looks up each
/// record's sound index in the object's sound table and calls the sound helper
/// with the record handle and a fixed enable flag. A helper result of two
/// returns two immediately; otherwise any zero result makes the final result
/// zero, while all other results leave it one.
lf_checker_rt::export!(thiscall, rw_009b2090(this: u32, slot_index: u32) -> u32 {
    const SLOT_STRIDE: u32 = 0x1a0;
    const RECORD_COUNT: u32 = 0x9c;
    const FIRST_RECORD: u32 = 0xa4;
    const RECORD_STRIDE: u32 = 0x14;
    const SOUND_TABLE: u32 = 0x5c;

    unsafe {
        let slot = this.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
        let count = ((slot + RECORD_COUNT) as *const u32).read_unaligned();
        let table = ((this + SOUND_TABLE) as *const u32).read_unaligned();
        let mut result = 1;
        for index in 0..count {
            let record = slot.wrapping_add(FIRST_RECORD).wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            let handle = (record as *const u32).read_unaligned();
            let sound_index = ((record + 0x0c) as *const u32).read_unaligned();
            let sound_id = ((table.wrapping_add(sound_index.wrapping_mul(4))) as *const u32).read_unaligned();
            let answer = lf_checker_rt::callee_thiscall!(1, u32, handle, sound_id, 1);
            if answer == 2 {
                return 2;
            }
            if answer == 0 {
                result = 0;
            }
        }
        result
    }
});
