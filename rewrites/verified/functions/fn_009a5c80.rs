// original: 0x009a5c80 script_audio_find_slot_by_thread
/// Finds the script audio record owned by a thread.
///
/// The receiver contains twenty 8-byte records beginning at `0x3A10`. The
/// first dword in each record is compared with the full thread identifier;
/// the method returns the first matching index or `-1` when none matches.
const SLOT_COUNT: u32 = 20;
const SLOTS_OFFSET: u32 = 0x3A10;
const SLOT_STRIDE: u32 = 8;

export!(thiscall, rw_009a5c80(this_: u32, thread_id: u32) -> i32 {
    for index in 0..SLOT_COUNT {
        let slot = this_
            .wrapping_add(SLOTS_OFFSET)
            .wrapping_add(index.wrapping_mul(SLOT_STRIDE));
        let owner = unsafe { (slot as *const u32).read() };
        if owner == thread_id {
            return index as i32;
        }
    }
    -1
});
