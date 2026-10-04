// original: 0x008D8A50 streaming_slot_initializer

const SENTINEL: u32 = 0xabcdabcd;
const TABLE_LEN: u32 = 1000;

/// Streaming-slot initializer: handles, zeroed state, two tables, globals.
pub fn init_slot(this: u32) -> u32 {
    unsafe {
        let base = this as *mut u8;
        let w32 = |off: u32, v: u32| (base.add(off as usize) as *mut u32).write(v);
        // Four handles from the same helper; only the last call is flagged.
        w32(0xfac, callee_cdecl!(0, u32, 0u32));
        w32(0xfb0, callee_cdecl!(0, u32, 0u32));
        w32(0xfb4, callee_cdecl!(0, u32, 0u32));
        w32(0xfb8, callee_cdecl!(0, u32, 1u32));
        // Scalar state to zero. The original clears [0xfd0, 0xff0) with
        // 8-byte vector stores (each also copies one adjacent stack word
        // that is zero under this contract's stack_fill), so the odd
        // words are zero here too.
        for off in [
            0xfc0u32, 0xfc4, 0xfd0, 0xfd4, 0xfd8, 0xfdc, 0xfe0, 0xfe4, 0xfe8, 0xfec,
        ] {
            w32(off, 0);
        }
        (base.add(0xfc8) as *mut u16).write(0);
        *base.add(0xfca) = 0;
        *base.add(0xfcc) = 0;
        // Sentinels outside the loop ranges, then the two tables to zero.
        w32(0xfa0, SENTINEL);
        w32(0xff0, SENTINEL);
        for i in 0..TABLE_LEN {
            w32(i * 4, 0);
            w32(0xff4 + i * 4, 0);
        }
        // Edge sentinels plus the published row pointers.
        let row_b = base.add(0xff4);
        (row_b as *mut u32).write(SENTINEL);
        global::<u32>(0x011737A8).write(row_b as u32);
        w32(0x0, SENTINEL);
        w32(0x1f90, SENTINEL);
        let row_a_end = base.add(0xf9c);
        (row_a_end as *mut u32).write(SENTINEL);
        global::<u32>(0x011737A4).write(row_a_end as u32);
        this
    }
}
