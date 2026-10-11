// original: 0x0062F7E0 streaming_find_pair_0062f7e0
/// Searches a signed-count table of record pointers for two matching words.
///
/// The object keeps its signed element count at +0x51B4 and a pointer to the record-pointer
/// table at +0x5134. Each record is inspected at +0x40 and
/// +0x44; the key supplies words at +0 and +4.
/// Counts at or below zero return -1. A positive count is scanned in ascending index order and the first
/// match is returned; a miss returns -1. This is a 32-bit thiscall operation with one pointer argument.
const COUNT_OFFSET: u32 = 0x51B4;
const TABLE_OFFSET: u32 = 0x5134;
const RECORD_KEY_A: u32 = 0x40;
const RECORD_KEY_B: u32 = 0x44;
const KEY_A: u32 = 0x0;
const KEY_B: u32 = 0x4;

lf_checker_rt::export!(thiscall, rw_0062f7e0(this_ptr: u32, key_ptr: u32) -> i32 {
    let signed_count = unsafe { (this_ptr.wrapping_add(COUNT_OFFSET) as *const i32).read_unaligned() };
    if signed_count <= 0 {
        return -1;
    }
    let wanted_a = unsafe { (key_ptr.wrapping_add(KEY_A) as *const u32).read_unaligned() };
    let wanted_b = unsafe { (key_ptr.wrapping_add(KEY_B) as *const u32).read_unaligned() };
    let table = unsafe { (this_ptr.wrapping_add(TABLE_OFFSET) as *const u32).read_unaligned() };
    for index in 0..(signed_count as u32) {
        let record = unsafe { (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() };
        let actual_a = unsafe { (record.wrapping_add(RECORD_KEY_A) as *const u32).read_unaligned() };
        let actual_b = unsafe { (record.wrapping_add(RECORD_KEY_B) as *const u32).read_unaligned() };
        if actual_a == wanted_a && actual_b == wanted_b {
            return index as i32;
        }
    }
    -1
});
