// original: 0x00963080 handle_register
/// Register a pointer in one of the two handle tables.
///
/// Arguments are `(sel, idx, ptr)`. A null pointer, a negative `idx`
/// (signed) or a low word at or above `0x818` (signed 16-bit) rejects the
/// call; otherwise `ptr` is stored at entry `(idx as i16)` of table A
/// (`0x1208970`) when `sel == 1` or table B (`0x12142D0`), with the high
/// word of `idx` kept as the tag. Note the signed bound admits a low word
/// of `0x8000..0xFFFF`, which addresses before the table; the rewrite keeps
/// that behaviour. Returns the entry index on success and the bound `0x818`
/// on a bound rejection; the other two rejections return the caller's EAX
/// unchanged, so the proof pins entry EAX to `0`.
lf_checker_rt::export!(cdecl, rw_00963080(sel: u32, idx: u32, ptr: u32) -> u32 {
    unsafe {
        const TABLE_A: u32 = 0x1208970;
        const TABLE_B: u32 = 0x12142d0;
        const COUNT: i16 = 0x818;
        const STRIDE: i32 = 8;
        if ptr == 0 {
            return 0;
        }
        if (idx as i32) < 0 {
            return 0;
        }
        let low = (idx & 0xffff) as u16 as i16;
        if low >= COUNT {
            return COUNT as u32;
        }
        let base = if sel == 1 { TABLE_A } else { TABLE_B };
        let e = lf_checker_rt::relocated(base)
            .wrapping_add(((low as i32).wrapping_mul(STRIDE)) as u32);
        (e as *mut u32).write_unaligned(ptr);
        (e.wrapping_add(4) as *mut u16).write_unaligned((idx >> 16) as u16);
        low as i32 as u32
    }
});
