// original: 0x00AD0510 audio_update_voice_flags (proposed)

/// Refresh a voice's flag word from the registry and its gate level.
///
/// Masks the flag word down to its kept bits, then sets the primary bit
/// when the voice key matches the registry's first slot or the secondary
/// bit on a match with any of the next three. The gate tag comes from the
/// registry's virtual gate (slot 5) applied to the key, truncated to a
/// half-word; the level lookup (thiscall/2: owner, 0, tag) reads the
/// level, and when it sits strictly between the two thresholds the hot
/// bit is set. Thiscall/0 (voice in ecx); returns the lookup answer.
lf_checker_rt::export!(thiscall, rw_00ad0510(this: u32) -> u32 {
    unsafe {
        const VT_GATE: u32 = 1;
        const LEVEL_OF: u32 = 2;
        const REGISTRY: u32 = 0x018B8968;
        const OWNER: u32 = 0x016D9F58;
        const FLAGS_OFF: u32 = 0x164;
        const KEY_OFF: u32 = 0xD8;
        const GATE_SLOT: u32 = 0x14;
        const TAG_OFF: u32 = 0x20;
        const KEEP_MASK: u32 = 0xE3FF_FFFF;
        const PRIMARY: u32 = 0x0800_0000;
        const SECONDARY: u32 = 0x0400_0000;
        const HOT: u32 = 0x1000_0000;
        const THRESH_A: u32 = 0x00FE88E8;
        const THRESH_B: u32 = 0x00FE8BB0;
        let at = this.wrapping_add(FLAGS_OFF);
        (at as *mut u32).write((at as *const u32).read() & KEEP_MASK);
        let reg = lf_checker_rt::global::<u32>(REGISTRY).read();
        let key = (this.wrapping_add(KEY_OFF) as *const u32).read();
        let mut f = (at as *const u32).read();
        if key == (reg.wrapping_add(0x3C) as *const u32).read() {
            f |= PRIMARY;
            (at as *mut u32).write(f);
        } else if key == (reg.wrapping_add(0x40) as *const u32).read()
            || key == (reg.wrapping_add(0x44) as *const u32).read()
            || key == (reg.wrapping_add(0x48) as *const u32).read()
        {
            f |= SECONDARY;
            (at as *mut u32).write(f);
        }
        let reg2 = lf_checker_rt::global::<u32>(REGISTRY).read();
        let vt = (reg2 as *const u32).read();
        let target = (vt.wrapping_add(GATE_SLOT) as *const u32).read();
        let gate: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let ans = gate(key);
        let tag = (ans.wrapping_add(TAG_OFF) as *const u16).read() as u32;
        let found = lf_checker_rt::callee_thiscall!(
            LEVEL_OF,
            u32,
            lf_checker_rt::relocated(OWNER),
            0u32,
            tag
        );
        let level = (found as *const f32).read();
        let ta = lf_checker_rt::global::<f32>(THRESH_A).read();
        if !(level > ta) {
            return found;
        }
        let tb = lf_checker_rt::global::<f32>(THRESH_B).read();
        if tb > level {
            (at as *mut u32).write((at as *const u32).read() | HOT);
        }
        found
    }
});
