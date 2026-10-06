// original: 0x0099DF40 audio_fire_voice_event (proposed)

/// Fire the voice event for the entity's active source, if any.
///
/// The source is `this`+0x60, falling back to `this`+0x64; with neither the
/// result is 0. Otherwise the bank byte at +0x48 selects: 0xFF fires the
/// event (callee 1, thiscall/1) with a zero voice id, any other value scales
/// the global rate by the bank byte (SIGNED 32-bit multiply, wrapping), adds
/// the table word indexed by the program byte at +0x40 (stride 0x6F40 from
/// the table base plus 0x6F10) and fires with that id. The event's answer is
/// returned. Thiscall with no stack words.
lf_checker_rt::export!(thiscall, rw_0099DF40(this: u32) -> u32 {
    unsafe {
        const SRC_A: u32 = 0x60;
        const SRC_B: u32 = 0x64;
        const PROGRAM: u32 = 0x40;
        const BANK: u32 = 0x48;
        const NO_BANK: u8 = 0xFF;
        const RATE: u32 = 0x0115D964;
        const TABLE_BASE: u32 = 0x0115D988;
        const STRIDE: u32 = 0x6F40;
        const TABLE_BIAS: u32 = 0x6F10;
        const EVENT_CALLEE: u32 = 1;
        let mut src = ((this.wrapping_add(SRC_A)) as *const u32).read_unaligned();
        if src == 0 {
            src = ((this.wrapping_add(SRC_B)) as *const u32).read_unaligned();
            if src == 0 {
                return 0;
            }
        }
        let bank = ((src.wrapping_add(BANK)) as *const u8).read();
        if bank == NO_BANK {
            return lf_checker_rt::callee_thiscall!(EVENT_CALLEE, u32, 0, 0);
        }
        let prog = ((src.wrapping_add(PROGRAM)) as *const u8).read() as u32;
        let rate = (lf_checker_rt::global::<u32>(RATE) as *const u32).read_unaligned();
        let base = (lf_checker_rt::global::<u32>(TABLE_BASE) as *const u32).read_unaligned();
        let scaled = (rate as i32).wrapping_mul(bank as i32) as u32;
        let cell = (base.wrapping_add(prog.wrapping_mul(STRIDE)).wrapping_add(TABLE_BIAS)
                    as *const u32).read_unaligned();
        let id = scaled.wrapping_add(cell);
        lf_checker_rt::callee_thiscall!(EVENT_CALLEE, u32, id, 0)
    }
});
