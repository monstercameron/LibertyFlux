// original: 0x00c07a40 stream_table_entry_or_default
/// Return the `index`-th table entry, or the shared default object.
///
/// The entry count is the 16-bit word at `this`+0x14 and the entry array is
/// the pointer at `this`+0x10. A signed in-range index returns the entry; an
/// out-of-range index returns the shared read-only default object. Thiscall:
/// `this` in ECX, one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c07a40(this: u32, index: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const TABLE_OFF: u32 = 0x10;
        const COUNT_OFF: u32 = 0x14;
        const DEFAULT_OBJ: u32 = 0x00EBDE98;
        let count = rd16(this + COUNT_OFF) as i32;
        if (index as i32) < count {
            let table = rd32(this + TABLE_OFF);
            rd32(table.wrapping_add((index as u32).wrapping_mul(4)))
        } else {
            lf_checker_rt::relocated(DEFAULT_OBJ)
        }
    }
});
