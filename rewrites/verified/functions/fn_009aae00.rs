// original: 0x009AAE00 audio_voice_flag_advance (proposed)

/// Voice-slot flag advance: moves one slot's flag from 2 ("ready") to 3.
///
/// Slot selector: `t = b + 1` (wrapping), `r = t div 3` remainder (signed,
/// truncation, matching x86 `idiv`), record index `r + 3 * a`, record size
/// 96. When the flag byte at `this + 96 * index + 8` equals 2 it is set to
/// 3; any other value is left alone. Division by the constant 3 never
/// faults. Returns nothing.
/// Original: 0x009AAE00 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_009AAE00(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const RECORD_SIZE: i32 = 96;
        const FLAG_OFF: i32 = 8;
        const FLAG_READY: u8 = 2;
        const FLAG_DONE: u8 = 3;
        let t = (b as i32).wrapping_add(1);
        let r = t % 3;
        let idx = r.wrapping_add((a as i32).wrapping_mul(3));
        let at = this
            .wrapping_add((idx.wrapping_mul(RECORD_SIZE)) as u32)
            .wrapping_add(FLAG_OFF as u32);
        let p = at as *mut u8;
        if p.read() == FLAG_READY {
            p.write(FLAG_DONE);
        }
        0
    }
});
