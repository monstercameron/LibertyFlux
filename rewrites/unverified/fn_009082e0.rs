// original: 0x009082e0 input_cell_release (proposed)
/// Release one grid cell unless the bounds callee objects.
///
/// Runs the bounds callee on `(x, y)` and returns its answer when non-zero.
/// Otherwise looks the cell up like the refresher (static dimension, mode
/// byte, offset and table; -1 returns -1), runs the check callee on
/// `(cell, flag)` (low byte must be set) and finally the release callee on
/// `(cell, flag, tag)` where `tag` is `0x1A` in mode and `0xA` otherwise,
/// returning its answer. Cdecl with two stack words.
export!(cdecl, rw_009082e0(x: u32, y: u32) -> u32 {
    unsafe {
        /// Static grid dimension (file VA).
        const DIM: u32 = 0x010344E4;
        /// Static mode byte (file VA).
        const MODE: u32 = 0x011609F6;
        /// Static index offset added when the mode byte is set (file VA).
        const OFF: u32 = 0x010344EC;
        /// Static cell table base (file VA).
        const BASE: u32 = 0x0118F4E8;
        /// Static flag word passed to two callees (file VA).
        const FLAG: u32 = 0x01032F58;
        const TAG_SET: u32 = 0x1A;
        const TAG_CLEAR: u32 = 0x0A;
        const BOUNDS_ID: u32 = 1;
        const CHECK_ID: u32 = 2;
        const RELEASE_ID: u32 = 3;
        let b: u32 = callee_cdecl!(BOUNDS_ID, u32, x, y);
        if b != 0 {
            return b;
        }
        let n = (global::<u32>(DIM)).read_unaligned();
        let mode = (global::<u8>(MODE)).read();
        let mut idx = n.wrapping_mul(y).wrapping_add(x);
        if mode != 0 {
            idx = idx.wrapping_add((global::<u32>(OFF)).read_unaligned());
        }
        let base = (global::<u32>(BASE)).read_unaligned();
        let cell = ((base.wrapping_add(idx.wrapping_mul(4))) as *const u32).read_unaligned();
        if cell == 0xFFFFFFFF {
            return cell;
        }
        let flag = (global::<u32>(FLAG)).read_unaligned();
        let tag = if mode != 0 { TAG_SET } else { TAG_CLEAR };
        let r: u32 = callee_cdecl!(CHECK_ID, u32, cell, flag);
        if (r as u8) == 0 {
            return r;
        }
        callee_cdecl!(RELEASE_ID, u32, cell, flag, tag)
    }
});
