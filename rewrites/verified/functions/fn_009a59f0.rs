// original: 0x009a59f0 script_audio_alloc_sound_id
/// Finds the first unused sound record in the script audio pool.
///
/// The pool contains one hundred records beginning at receiver offset
/// `0x33D8`, each `0x10` bytes apart. The `0xFFFF_FFFF` sentinel marks the first free
/// record; the method returns its zero-based index, or `-1` when every record
/// is occupied.
const RECORDS: u32 = 100;
const RECORDS_OFFSET: u32 = 0x33D8;
const RECORD_STRIDE: u32 = 0x10;

export!(thiscall, rw_009a59f0(this_: u32) -> i32 {
    for index in 0..RECORDS {
        let record = this_
            .wrapping_add(RECORDS_OFFSET)
            .wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        let sound_id = unsafe { (record as *const u32).read() };
        if sound_id == u32::MAX {
            return index as i32;
        }
    }
    -1
});
