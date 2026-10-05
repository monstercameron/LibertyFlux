// original: 0x009BAD70 CCamScriptInstruction_SetScriptModeActive::vf2
/// Update the script-mode flag bits, notify, and publish the mode byte.
///
/// Resolves the camera (`callee 1`, thiscall/0, no null check). Bit 0 of the
/// byte at `this+0x08` is written into bit 2 of the flag byte at camera
/// `+0x13C`, then bit 0 of the byte at `this+0x09` into bit 3 (other bits
/// preserved). `callee 2` (thiscall/0) is notified with the camera, and the
/// mode byte is copied to its global.
lf_checker_rt::export!(thiscall, rw_009BAD70(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const MODE: u32 = 0x08;
        const MODE2: u32 = 0x09;
        const FLAGS: u32 = 0x13C;
        const BIT_ACTIVE: u8 = 0x04;
        const BIT_OTHER: u8 = 0x08;
        const MODE_GLOBAL: u32 = 0x128E932;
        const LOOKUP: u32 = 1;
        const NOTIFY: u32 = 2;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let fs = cam.wrapping_add(FLAGS) as *mut u8;
        let b0 = (this.wrapping_add(MODE) as *const u8).read();
        let o0 = fs.read();
        fs.write(if b0 & 1 != 0 { o0 | BIT_ACTIVE } else { o0 & !BIT_ACTIVE });
        let b1 = (this.wrapping_add(MODE2) as *const u8).read();
        let o1 = fs.read();
        fs.write(if b1 & 1 != 0 { o1 | BIT_OTHER } else { o1 & !BIT_OTHER });
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, cam);
        lf_checker_rt::global::<u8>(MODE_GLOBAL).write(b0);
        0
    }
});
