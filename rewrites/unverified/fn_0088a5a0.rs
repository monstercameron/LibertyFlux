// original: 0x0088A5A0 stream_resource_get_chunk_index (proposed)

/// Index of the chunk containing a key, or 0xff for a null key.
///
/// `this` points to the resource manager: table base at `+0xe8`, chunk size
/// (the unsigned divisor) at `+0xc4`. When `key` is 0 the result is 0xff.
/// Otherwise the group's base at `group * 0x6f40 + base + 0x6f10` is
/// subtracted from `key` (wrapping unsigned) and divided (unsigned) by the
/// chunk size. A zero chunk size would fault the original's divide; the proof
/// only runs nonzero sizes.
///
/// Original: 0x0088A5A0 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_0088a5a0(this: u32, group: u32, key: u32) -> u32 {
    unsafe {
        const GROUP_STRIDE: u32 = 0x6f40;
        const TABLE_BASE: u32 = 0xe8;
        const GROUP_BASE_OFF: u32 = 0x6f10;
        const CHUNK_SIZE_OFF: u32 = 0xc4;
        const NULL_KEY: u32 = 0xff;

        if key == 0 {
            return NULL_KEY;
        }
        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let size = ((this + CHUNK_SIZE_OFF) as *const u32).read_unaligned();
        let lo = (group.wrapping_mul(GROUP_STRIDE).wrapping_add(base).wrapping_add(GROUP_BASE_OFF)
            as *const u32)
            .read_unaligned();
        // size is nonzero on every proven trial, as the original requires.
        key.wrapping_sub(lo) / size
    }
});
