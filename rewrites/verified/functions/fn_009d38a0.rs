// original: 0x009D38A0 indexed_byte_combine (proposed)
//
/// Looks up an index through a helper, then combines it with a table byte.
///
/// Calls the indexer (callee) with `key`, reads one byte at `table + index`
/// (`table` is at `this + 0x04`) and returns `(index << 8) | byte`. Thiscall,
/// one stack word.
lf_checker_rt::export!(thiscall, rw_009D38A0(this: u32, key: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x04;
        const INDEXER: u32 = 1;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let index: u32 = lf_checker_rt::callee_thiscall!(INDEXER, u32, this, key);
        let table = rd(this.wrapping_add(TABLE));
        let b = (table.wrapping_add(index) as *const u8).read();
        index.wrapping_shl(8).wrapping_add(b as u32)
    }
});
