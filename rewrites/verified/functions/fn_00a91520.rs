// original: 0x00a91520 stream_entry_init

/// Streaming entry initializer: copies 16 bytes and zeroes the tail.
///
/// `this` (ECX) points to a >= 0x28-byte record, `src` to 16 readable bytes,
/// `tag` is stored at `this+0x24`. Copies `src[0..16]` to `this[0..16]`,
/// writes 0 to `this+0x10, +0x14, +0x18, +0x1c, +0x20`, stores `tag` at
/// `this+0x24`, returns `this`. No calls, no globals.
/// Original: 0x00A91520 (thiscall, ECX + two stack words), 69 bytes.
lf_checker_rt::export!(thiscall, rw_00a91520(this: u32, src: u32, tag: u32) -> u32 {
    unsafe {
        const COPY_LEN: usize = 16;
        const ZERO_OFFS: [u32; 5] = [0x10, 0x14, 0x18, 0x1c, 0x20];
        const TAG_OFF: u32 = 0x24;
        core::ptr::copy_nonoverlapping(src as *const u8, this as *mut u8, COPY_LEN);
        for off in ZERO_OFFS {
            (this.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(TAG_OFF) as *mut u32).write_unaligned(tag);
        this
    }
});
