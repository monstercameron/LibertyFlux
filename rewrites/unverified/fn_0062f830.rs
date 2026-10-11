// original: 0x0062F830 streaming_find_record_key_0062f830
/// Searches a signed-count table of record pointers using four independently sized key fields.
///
/// The signed count is at +0x57C0; the inline pointer table begins at +0x5740. The
/// two dword and two word fields are compared in the same order as the original. Nonpositive signed
/// counts and misses return -1; the first matching row returns its zero-based index. This is a 32-bit
/// thiscall operation with one pointer argument.
const COUNT_OFFSET: u32 = 0x57C0;
const TABLE_OFFSET: u32 = 0x5740;
const KEY_DWORD_A: u32 = 0x0;
const RECORD_DWORD_A: u32 = 0x4;
const KEY_WORD_A: u32 = 0x4;
const RECORD_WORD_A: u32 = 0x8;
const KEY_DWORD_B: u32 = 0x8;
const RECORD_DWORD_B: u32 = 0xC;
const KEY_WORD_B: u32 = 0xC;
const RECORD_WORD_B: u32 = 0x10;

lf_checker_rt::export!(thiscall, rw_0062f830(this_ptr: u32, key_ptr: u32) -> i32 {
    let signed_count = unsafe { (this_ptr.wrapping_add(COUNT_OFFSET) as *const i32).read_unaligned() };
    if signed_count <= 0 {
        return -1;
    }
    let wanted_0 = unsafe { (key_ptr.wrapping_add(KEY_DWORD_A) as *const u32).read_unaligned() };
    let wanted_1 = unsafe { (key_ptr.wrapping_add(KEY_WORD_A) as *const u16).read_unaligned() };
    let wanted_2 = unsafe { (key_ptr.wrapping_add(KEY_DWORD_B) as *const u32).read_unaligned() };
    let wanted_3 = unsafe { (key_ptr.wrapping_add(KEY_WORD_B) as *const u16).read_unaligned() };
    let table = this_ptr.wrapping_add(TABLE_OFFSET);
    for index in 0..(signed_count as u32) {
        let record = unsafe { (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() };
        let actual_0 = unsafe { (record.wrapping_add(RECORD_DWORD_A) as *const u32).read_unaligned() };
        let actual_1 = unsafe { (record.wrapping_add(RECORD_WORD_A) as *const u16).read_unaligned() };
        let actual_2 = unsafe { (record.wrapping_add(RECORD_DWORD_B) as *const u32).read_unaligned() };
        let actual_3 = unsafe { (record.wrapping_add(RECORD_WORD_B) as *const u16).read_unaligned() };
        if actual_1 == wanted_1 && actual_0 == wanted_0 && actual_3 == wanted_3 && actual_2 == wanted_2 {
            return index as i32;
        }
    }
    -1
});
