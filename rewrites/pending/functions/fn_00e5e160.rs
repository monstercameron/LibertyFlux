// original: 0x00e5e160 net_state_clear_e160
/// Clear a mixed-width network state record.
///
/// Zeroes 24 fields (dwords, words and bytes, some at unaligned offsets)
/// spanning `0x019D3030..0x019D308C`, in ascending order. Computes no result
/// and leaves the return register untouched.
export!(cdecl, rw_00e5e160() -> () {
    unsafe {
        /// Start of the state record (file VA).
        const BASE: u32 = 0x019D3030;
        /// (byte offset, width in bytes) of each cleared field, in order.
        const STORES: [(usize, u8); 24] = [
            (0x00, 4), (0x04, 4), (0x0A, 4), (0x0E, 4), (0x12, 1), (0x15, 4),
            (0x19, 4), (0x1D, 2), (0x21, 4), (0x25, 4), (0x29, 1), (0x2C, 4),
            (0x30, 4), (0x34, 2), (0x38, 4), (0x3C, 4), (0x40, 1), (0x43, 4),
            (0x47, 4), (0x4B, 2), (0x4F, 4), (0x53, 4), (0x57, 1), (0x5A, 2),
        ];
        let base = relocated(BASE) as *mut u8;
        let mut i = 0;
        while i < STORES.len() {
            let (off, width) = STORES[i];
            let p = base.add(off);
            if width == 4 {
                (p as *mut u32).write_unaligned(0);
            } else if width == 2 {
                (p as *mut u16).write_unaligned(0);
            } else {
                p.write(0);
            }
            i += 1;
        }
    }
});
