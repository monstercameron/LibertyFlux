// original: 0x009a8a50 audScriptAudioEntity::vf2
/// Tear down the script audio entity's helpers, then its base.
///
/// Releases each helper that is present: the cached handle at
/// `this+0x3ac4` (stubbed, thiscall/1 with 0), the cached block at
/// `this+0x3ac0` (stubbed, thiscall/0) which is then cleared, and
/// the cached list at `this+0x3acc` (stubbed, thiscall/1 with the
/// field's own address) which is then cleared. Then the entity's
/// own words are reset (`+0x3ac8` and `+0x3adc` to 0, `+0x3ad0`,
/// `+0x3ad4` and `+0x3ad8` to -1), the state object's closer at its
/// `+8` slot is invoked with `this+0x2be0` in ECX (the slot holds
/// the stub address in fabricated heap), and control tail-jumps to
/// the base destructor (stubbed) with the object in ECX; the
/// rewrite ends in the same stub call. Thiscall, no stack words,
/// dword result (the base destructor's answer).
export!(thiscall, rw_009A8A50(this: u32) -> u32 {
    unsafe {
        let h = ((this + 0x3ac4) as *const u32).read_unaligned();
        if h != 0 {
            let _: u32 = callee_thiscall!(1, u32, h, 0);
        }
        let b = ((this + 0x3ac0) as *const u32).read_unaligned();
        if b != 0 {
            let _: u32 = callee_thiscall!(2, u32, b);
            ((this + 0x3ac0) as *mut u32).write_unaligned(0);
        }
        let l = ((this + 0x3acc) as *const u32).read_unaligned();
        if l != 0 {
            let _: u32 = callee_thiscall!(3, u32, l, this + 0x3acc);
            ((this + 0x3acc) as *mut u32).write_unaligned(0);
        }
        let st = ((this + 0x2be0) as *const u32).read_unaligned();
        ((this + 0x3ac8) as *mut u32).write_unaligned(0);
        ((this + 0x3ad8) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + 0x3ad0) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + 0x3ad4) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + 0x3adc) as *mut u32).write_unaligned(0);
        let _tgt = ((st + 8) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(4, u32, this + 0x2be0);
        callee_thiscall!(5, u32, this)
    }
});
