// original: 0x009a38c0 audio_poll_focus_and_tick
/// Original 0x009a38c0 (unnamed): poll audio focus and dispatch the tick.
///
/// When the focus gate is set, asks the focus hook (reached through a data
/// slot) whether audio is live; a null answer falls back to two flag bytes.
/// Live audio latches the saved mode byte and the armed flag, while idle
/// audio restores the saved mode and clears the flag. Then, unless a global
/// stop word, a tag mismatch, or the idle tag says otherwise, an optional
/// pre-tick runs (skipped when the mode byte of `arg` is clear), the mixer
/// tick runs on the shared mixer object, and the entity tick runs on `this`;
/// any veto instead runs the idle handler on `this`. Returns the tick answer.
export!(thiscall, rw_009a38c0(this_: u32, arg: u32) -> u32 {
    type SlotFn = extern "thiscall" fn(u32, u32) -> u32;
    let gate = unsafe { (relocated(0x011609F6) as *const u8).read() };
    let mut idle = gate == 0;
    if !idle {
        let slot = unsafe { (relocated(0x00E733DC) as *const u32).read() };
        let hook: SlotFn = unsafe { core::mem::transmute(slot as usize) };
        let g = unsafe { (relocated(0x017ACCD8) as *const u32).read() };
        let ans = hook(this_, g);
        let mut al = if ans != 0 {
            1u8
        } else if unsafe { (relocated(0x0105B48F) as *const u8).read() } == 0 {
            0u8
        } else if unsafe { (relocated(0x017ED8D1) as *const u8).read() } != 0 {
            1u8
        } else {
            0u8
        };
        al |= unsafe { (relocated(0x01173590) as *const u8).read() };
        al |= unsafe { (relocated(0x01173591) as *const u8).read() };
        idle = al == 0;
        if !idle && unsafe { ((this_ + 0x8a) as *const u8).read() } == 0 {
            let m = unsafe { (relocated(0x012845C8) as *const u8).read() };
            unsafe {
                ((this_ + 0x87) as *mut u8).write(m);
                (relocated(0x012845C8) as *mut u8).write(1);
                ((this_ + 0x8a) as *mut u8).write(1);
            }
        }
    }
    if idle {
        if unsafe { ((this_ + 0x8a) as *const u8).read() } != 0 {
            let m = unsafe { ((this_ + 0x87) as *const u8).read() };
            unsafe { (relocated(0x012845C8) as *mut u8).write(m) };
        }
        unsafe { ((this_ + 0x8a) as *mut u8).write(0) };
    }
    if unsafe { (relocated(0x011F7060) as *const u32).read() } == 1 {
        return callee_thiscall!(5, u32, this_);
    }
    let a = unsafe { (relocated(0x012088B4) as *const u32).read() };
    if a != unsafe { (relocated(0x00F1C040) as *const u32).read() } {
        return callee_thiscall!(5, u32, this_);
    }
    if unsafe { (relocated(0x01037720) as *const u32).read() } == 0x12 {
        return callee_thiscall!(5, u32, this_);
    }
    if arg & 0xff != 0
        && callee_thiscall!(2, u32, this_) as u8 != 0
    {
        return callee_thiscall!(5, u32, this_);
    }
    if callee_thiscall!(3, u32, relocated(0x0128E94C)) as u8 != 0 {
        return callee_thiscall!(5, u32, this_);
    }
    callee_thiscall!(4, u32, this_)
});
