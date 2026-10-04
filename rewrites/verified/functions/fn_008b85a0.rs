// original: 0x008b85a0 pad_presence_scan
/// Present-pad scan.
///
/// Returns the lowest pad index strictly above `after` (scanning up to 0x40)
/// whose presence probe reports attached, or -1 when no later pad is present.
export!(cdecl, rw_008b85a0(after: u32) -> u32 {
    unsafe {
        const PADS: u32 = 0x01BB5624;
        const COUNT: u32 = 0x40;
        let pads = (relocated(PADS) as *const u32).read();
        let mut i = after.wrapping_add(1);
        if (i as i32) >= (COUNT as i32) {
            return 0xFFFF_FFFF;
        }
        loop {
            let present: u32 = callee_thiscall!(2, u32, pads, i);
            if (present as u8) != 0 {
                return i;
            }
            i = i.wrapping_add(1);
            if !((i as i32) < (COUNT as i32)) {
                return 0xFFFF_FFFF;
            }
        }
    }
});
