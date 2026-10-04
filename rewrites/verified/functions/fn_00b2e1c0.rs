// original: 0x00b2e1c0 garage_watchdog_high
// 0xB2E1C0 garage_watchdog_high (thiscall/0 -> al).
//
// Same watchdog with a high measurement band and unity scale: the fire path
// additionally re-arms the scaler before firing the pair setter.
export!(thiscall, rw_00b2e1c0(rec: *mut u8) -> u8 {
    unsafe {
        const WINDOW: u32 = 0x1388;
        const BAND_TOP: f32 = f32::from_bits(0x3F7AE148);
        const BAND_MID: f32 = f32::from_bits(0x3F000000);
        const UNITY: u32 = 0x3F800000;
        let set_pair: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let set_scaled: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let measure: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(callee_addr(3) as usize);
        *rec.add(0x4C) |= 0x20;
        let mut flags = *rec.add(0x4C);
        let mut expired = false;
        if flags & 0x10 == 0 {
            *(rec.add(0x44) as *mut u32) = *global::<u32>(0x11735B4);
        }
        let deadline = (*(rec.add(0x44) as *const u32)).wrapping_add(WINDOW);
        if *global::<u32>(0x11735B4) > deadline {
            flags &= !0x20;
            expired = true;
            *rec.add(0x4C) = flags;
        }
        let target = *(rec.add(0x50) as *const u32);
        if target == 0 || *((target + 0x38) as *const u32) == 0 {
            return 0;
        }
        set_pair(target, 0, 0);
        set_scaled(target, UNITY, 0);
        let v = measure(rec as u32);
        if expired || v > BAND_TOP {
            set_scaled(target, UNITY, 1);
            set_pair(target, 1, 0);
            return 1;
        }
        let tick = *global::<u32>(0x1173604);
        if !(v > BAND_MID) || *(rec.add(0x54) as *const f32) < v {
            *(rec.add(0x54) as *mut f32) = v;
            *(rec.add(0x58) as *mut u32) = tick;
            return 0;
        }
        if *(rec.add(0x58) as *const u32) == tick.wrapping_sub(1) {
            set_scaled(target, UNITY, 1);
            set_pair(target, 1, 0);
            return 1;
        }
        *(rec.add(0x54) as *mut f32) = v;
        *(rec.add(0x58) as *mut u32) = tick;
        0
    }
});
