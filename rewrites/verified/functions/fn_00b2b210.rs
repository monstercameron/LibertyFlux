// original: 0x00b2b210 obj_mode_dispatch

/// Dispatches an object by its signed mode byte through a jump table.
///
/// Reads the signed mode byte at `a1 + 0x26`. When it is above 28 as an
/// UNSIGNED value (so negative bytes take this path too), returns the
/// sign-extended byte and calls nothing. When the table maps the byte to the
/// skip entry, returns 2 (the table index: the index load overwrote the
/// byte in EAX before the shared epilogue) and calls nothing. Otherwise
/// entry 0 tail-calls the record refresh with `a0` and returns its answer;
/// entry 1 builds a 28-byte scratch block, calls the prepare callee
/// with (block, `a0`), then calls the rebuild with (`a0`, block, state-is-2)
/// and returns its answer. The table maps bytes 0..=28 to
/// [0,0,1,1,1,0,1,1,0,1,1,1,1,0,1,1,0,0,0,0,0,0,2,2,2,2,0,2,0].
/// Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b2b210(a0: u32, a1: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x26;
        const MODE_MAX: u32 = 0x1C;
        const STATE_OFF: u32 = 0x1304;
        const REFRESH: u32 = 0;
        const PREPARE: u32 = 1;
        const REBUILD: u32 = 2;
        const ROUTE: [u8; 29] = [
            0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2, 2,
            2, 0, 2, 0,
        ];
        let b = ((a1 + MODE_OFF) as *const u8).read();
        if (b as u32) > MODE_MAX {
            return (b as i8) as i32 as u32;
        }
        match ROUTE[b as usize] {
            0 => lf_checker_rt::callee_cdecl!(REFRESH, u32, a0),
            1 => {
                // The original computes the two block pointers with the same
                // lea at different stack depths, so the rebuild sees the
                // block 8 bytes lower than the prepare filled: two zero
                // words (untouched scratch) followed by the first five
                // prepared words.
                let mut block = [0u32; 9];
                let bp = block.as_mut_ptr() as u32;
                lf_checker_rt::callee_cdecl!(PREPARE, u32, bp.wrapping_add(8), a0);
                let state = ((a0 + STATE_OFF) as *const u32).read_unaligned();
                let flag = if state == 2 { 1u32 } else { 0u32 };
                lf_checker_rt::callee_cdecl!(REBUILD, u32, a0, bp, flag)
            }
            _ => 2,
        }
    }
});
