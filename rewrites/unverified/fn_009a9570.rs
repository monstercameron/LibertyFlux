// original: 0x009a9570 ring_slot_append
/// Append a four-word record to the ring at the cursor, then advance it.
///
/// The ring holds 70 records of four dwords starting at `this+0x56c`.
/// The cursor at `this+0x9cc` selects the record: the four arguments are
/// stored into its four words (the cursor is re-read before each store,
/// as the original does), then the cursor advances by one modulo 70.
/// Thiscall, four stack words, no result.
export!(thiscall, rw_009A9570(this: u32, w0: u32, w1: u32, w2: u32, w3: u32) -> u32 {
    unsafe {
        const RING: u32 = 0x56c;
        const CURSOR: u32 = 0x9cc;
        const COUNT: u32 = 70;
        const STRIDE: u32 = 16;
        let rd = |off: u32| ((this + off) as *const u32).read_unaligned();
        let wr = |off: u32, v: u32| ((this + off) as *mut u32).write_unaligned(v);
        // The four stores land on consecutive words: +0x56c, +0x570,
        // +0x574, +0x578 relative to the record start.
        wr(RING + rd(CURSOR) * STRIDE, w0);
        wr(RING + 4 + rd(CURSOR) * STRIDE, w1);
        wr(RING + 8 + rd(CURSOR) * STRIDE, w2);
        wr(RING + 12 + rd(CURSOR) * STRIDE, w3);
        let next = rd(CURSOR).wrapping_add(1) % COUNT;
        wr(CURSOR, next);
        0
    }
});
