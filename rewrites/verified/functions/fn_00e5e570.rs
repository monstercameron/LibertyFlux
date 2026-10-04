// original: 0x00e5e570 net_state_block_copy_570
/// Copy the six-word state block to its shadow and return the last word.
///
/// The original moves six consecutive dwords from the live state block to
/// the shadow block, one at a time through EAX, and returns the sixth word
/// (the value left in EAX). Both blocks are plain globals.
export!(cdecl, rw_00e5e570() -> u32 {
    unsafe {
        /// Live state block, six dwords (file VA).
        const SRC: u32 = 0x01046540;
        /// Shadow block it is copied to (file VA).
        const DST: u32 = 0x0110E8A0;
        /// Words in the block.
        const WORDS: u32 = 6;
        let src = global::<u32>(SRC);
        let dst = global::<u32>(DST);
        let mut last: u32 = 0;
        let mut i: u32 = 0;
        while i < WORDS {
            last = src.add(i as usize).read();
            dst.add(i as usize).write(last);
            i += 1;
        }
        last
    }
});
