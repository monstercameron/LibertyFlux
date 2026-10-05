// original: 0x008A9EF0 audio_voice_update (proposed)

/// Refresh voice activations, then notify every live voice of a change.
///
/// Prelude: a clear flag at `this+0x3230` (or a clear flag at `this+0x3231`
/// with global `0x115f850` clear) means there is nothing to do. An active
/// voice with the global clear is deactivated (voice reset `0x8a9580`, then
/// the lock/unlock pair around a decrement of the shared counter
/// `0x115f854`) and the function returns. An inactive voice with the global
/// set is activated (lock/unlock around an increment) and falls through.
/// The main loop then scans voice slots 1..799 against the bitset at
/// `this+0x28a0` (word `i >> 5`, bit `i`): for each set bit, when global
/// `0x115dbe4` is set the slot's probe hook (vtable slot `+0x14` of the
/// object at `this+0xfa8+8*(i-1)`, thiscall, no arguments) runs first and a
/// zero answer skips the slot; otherwise the slot's notify hook (vtable slot
/// `+0xc`, thiscall with the function's argument word) runs. No return
/// value. Original is thiscall with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008A9EF0(this: u32, arg: u32) -> u32 {
    const RESET: u32 = 1;
    const LOCK: u32 = 2;
    const UNLOCK: u32 = 3;
    const PROBE: u32 = 4;
    const NOTIFY: u32 = 5;
    const LIVE: u32 = 0x3230;
    const ACTIVE: u32 = 0x3231;
    const BITSET: u32 = 0x28a0;
    const SLOTS: u32 = 0xfa8;
    const HANDLE: u32 = 0x0115_f858;
    const COUNTER: u32 = 0x0115_f854;
    const GATE: u32 = 0x0115_f850;
    const FILTER: u32 = 0x0115_dbe4;
    const PROBE_SLOT: u32 = 0x14;
    const NOTIFY_SLOT: u32 = 0x0c;
    const VOICES: u32 = 0x320;
    unsafe {
        #[inline(always)]
        unsafe fn locked(counter_delta: i32) {
            unsafe {
                let h = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
                lf_checker_rt::callee_cdecl!(LOCK, u32, h);
                let h2 =
                    (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
                let c = lf_checker_rt::global::<u32>(COUNTER);
                let v = (c as *const u32).read_unaligned();
                (c as *mut u32).write_unaligned(
                    if counter_delta < 0 { v.wrapping_sub(1) } else { v.wrapping_add(1) });
                lf_checker_rt::callee_cdecl!(UNLOCK, u32, h2);
            }
        }
        if ((this + LIVE) as *const u8).read() == 0 {
            return 0;
        }
        if ((this + ACTIVE) as *const u8).read() != 0 {
            if ((lf_checker_rt::global::<u8>(GATE)) as *const u8).read() == 0 {
                lf_checker_rt::callee_thiscall!(RESET, u32, this);
                locked(-1);
                ((this + ACTIVE) as *mut u8).write(0);
                return 0;
            }
        } else {
            if ((lf_checker_rt::global::<u8>(GATE)) as *const u8).read() == 0 {
                return 0;
            }
            locked(1);
            ((this + ACTIVE) as *mut u8).write(1);
        }
        let filter = (lf_checker_rt::global::<u8>(FILTER) as *const u8).read();
        let bits = ((this + BITSET) as *const u32).read_unaligned();
        let mut mask = 2u32;
        let mut slot = this + SLOTS;
        let mut i = 1u32;
        while i < VOICES {
            let w = ((bits + (i >> 5) * 4) as *const u32).read_unaligned();
            if w & mask != 0 {
                let mut run = true;
                if filter != 0 {
                    let obj = (slot as *const u32).read_unaligned();
                    let vt = (obj as *const u32).read_unaligned();
                    let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        ((vt + PROBE_SLOT) as *const u32).read_unaligned() as usize);
                    run = f(obj) as u8 != 0;
                }
                if run {
                    let obj = (slot as *const u32).read_unaligned();
                    let vt = (obj as *const u32).read_unaligned();
                    let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        ((vt + NOTIFY_SLOT) as *const u32).read_unaligned() as usize);
                    f(obj, arg);
                }
            }
            mask = mask.rotate_left(1);
            slot += 8;
            i += 1;
        }
    }
    0
});
