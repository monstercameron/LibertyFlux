// original: 0x00974390 audio_indexed_row_store (proposed)

/// Store one source row into the indexed table block.
///
/// The id in `slot` selects the row (each row is 48 bytes at +0x3050); an id
/// of -1 allocates one through the allocator callee first, and a second -1
/// returns early with -1. Twelve words are copied from `src`, then the last
/// word is overwritten with the global stamp. Returns the stamp, or -1 on
/// the early path. Both id compares are equality.
/// Original: 0x00974390 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00974390(this: u32, slot: u32, src: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x3050;
        const ROW: u32 = 48;
        const WORDS: u32 = 12;
        const STAMP: u32 = 0x11735B4;
        const ALLOC: u32 = 1;
        let cell = slot as *mut u32;
        let mut idx = cell.read_unaligned();
        if idx == 0xFFFFFFFF {
            let r = lf_checker_rt::callee_thiscall!(ALLOC, u32, this);
            cell.write_unaligned(r);
            if r == 0xFFFFFFFF {
                return 0xFFFFFFFF;
            }
            idx = r;
        }
        let dst = this
            .wrapping_add(idx.wrapping_mul(6).wrapping_mul(8))
            .wrapping_add(BASE);
        for i in 0..WORDS {
            let w = ((src.wrapping_add(i * 4)) as *const u32).read_unaligned();
            ((dst.wrapping_add(i * 4)) as *mut u32).write_unaligned(w);
        }
        let g = lf_checker_rt::global::<u32>(STAMP).read_unaligned();
        ((dst.wrapping_add(0x2C)) as *mut u32).write_unaligned(g);
        g
    }
});
