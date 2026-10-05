// original: 0x00a91190 stream_indexed_store

/// Stores a value into a callee-selected record slot.
///
/// Calls the selector (callee 1, thiscall) with (`a2`, `a3`); its answer is
/// an index into the record array at `this+0x80` with 96-byte stride, and
/// `val` is written at index position `+0x50`. Returns `val`. One call.
/// Original: 0x00A91190 (thiscall, ECX + three stack words), 40 bytes.
lf_checker_rt::export!(thiscall, rw_00a91190(this: u32, val: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const ARR_OFF: u32 = 0x80;
        const SLOT_OFF: u32 = 0x50;
        const STRIDE: u32 = 96;
        const SELECTOR: u32 = 1;
        let idx: u32 = lf_checker_rt::callee_thiscall!(SELECTOR, u32, this, a2, a3);
        let arr = (this.wrapping_add(ARR_OFF) as *const u32).read_unaligned();
        let slot = arr
            .wrapping_add(idx.wrapping_mul(STRIDE))
            .wrapping_add(SLOT_OFF);
        (slot as *mut u32).write_unaligned(val);
        val
    }
});
