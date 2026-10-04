// original: 0x00a95bd0 streaming_device_poll
/// Polls every streaming slot and pokes the live ones through their sink.
///
/// Walks the 255 global streaming records; a record whose in-use byte and
/// tag byte are both set is resolved through the locator, then poked once
/// through its sink's function table when its state byte reads 0, or once
/// when it reads 1. Returns nothing meaningful.
export!(cdecl, rw_00a95bd0() -> u32 {
    unsafe {
        unsafe fn poke(r: u32, s: u32) {
            let vt = *(r as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((vt + 0x4C) as *const u32) as usize);
            f(r, s);
        }
        let mut s = relocated(0x12FB3C8);
        let end = relocated(0x1305328);
        while (s as i32) < (end as i32) {
            if *((s + 0x8C) as *const u8) != 0 && *(s as *const u8) != 0 {
                let r = callee_cdecl!(1, u32, s, 1);
                if *((s + 0x8D) as *const u8) == 0 {
                    poke(r, s);
                }
                if *((s + 0x8D) as *const u8) == 1 {
                    poke(r, s);
                }
            }
            s = s.wrapping_add(0xA0);
        }
        0
    }
});
