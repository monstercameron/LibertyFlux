// original: 0x009A68C0 audio_update_flag_2c (proposed)

/// Conditionally copies a flag byte into audio global 0x2C.
///
/// thiscall: `this` points to the audio object, `value` supplies the byte,
/// `slot` selects. A negative `slot` (signed test) leaves the global alone.
/// Otherwise the global keeps its old byte unless `slot` equals the object's
/// dword at `SLOT_REF` (+0x3ABC), in which case the low byte of `value` is
/// stored. Leaf: no calls. Identical to 0x009A6860 except the global.
/// The return register is the input byte on the taken path and the caller's
/// leftover on the skipped path, so the contract compares no return channel.
lf_checker_rt::export!(thiscall, rw_009a68c0(this: u32, value: u32, slot: u32) -> u32 {
    unsafe {
        const SLOT_REF: u32 = 0x3ABC;
        const FLAG: u32 = 0x00116252C;
        if (slot as i32) >= 0 {
            let current = lf_checker_rt::global::<u8>(FLAG).read();
            let wanted = (this.wrapping_add(SLOT_REF) as *const u32).read_unaligned();
            let next = if slot == wanted { value as u8 } else { current };
            lf_checker_rt::global::<u8>(FLAG).write(next);
        }
    }
    0
});
