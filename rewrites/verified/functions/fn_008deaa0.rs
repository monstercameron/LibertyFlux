// original: 0x008deaa0 CDataBlockDC::vf2

/// Size of a data block rounded up for 16-byte alignment plus one header row.
///
/// `this` points to the block descriptor; the stored size is the word at
/// `+0x08`. Returns `size + 16 + ((-size) mod 16)`, i.e. the next
/// 16-byte boundary at or after `size`, plus a further 16 bytes. No calls,
/// no writes; thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_008deaa0(this: u32) -> u32 {
    unsafe {
        const SIZE_OFF: u32 = 0x08;
        const ALIGN: u32 = 0x10;
        const ALIGN_MASK: u32 = 0x0f;
        let size = ((this + SIZE_OFF) as *const u32).read_unaligned();
        let pad = size.wrapping_neg() & ALIGN_MASK;
        size.wrapping_add(ALIGN).wrapping_add(pad)
    }
});
