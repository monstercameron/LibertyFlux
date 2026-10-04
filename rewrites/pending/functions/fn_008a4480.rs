// original: 0x008a4480 rage::audVariableCurveSound::vf7
/// Prepare an `audVariableCurveSound` for playback (vf7).
///
/// Original 0x008A4480 (`thiscall(this, a1, a2, a3)`): runs the base
/// prepare; returns its answer unless its low byte is set. Otherwise, when
/// the info pointer at `[this+0x94]` is non-null, resolves a selector
/// through the audio heap callee (0xFF when it answers 0, else the bank
/// division `(ans - entry) / stride`, low byte) and stores it at
/// `this+0x48`; runs the curve member's prepare at `this+0xB8`; when the
/// flag at `this+0xDE` is set, resolves two pointers through the object's
/// own vtable slot +0x10 into `this+0xB0`/`this+0xB4` and returns whether
/// both are non-null. Every tail return writes only AL, so the last
/// callee answer's high bytes survive (reproduced with `& 0xFFFFFF00`).
export!(thiscall, rw_008a4480(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {    let ok = callee_thiscall!(1, u32, this, a1, a2, a3);
    if ok & 0xFF == 0 {
        return ok;
    }
    let info = unsafe { ((this + 0x94) as *const u32).read_unaligned() };
    let w0 = unsafe { (info as *const u32).read_unaligned() };
    if w0 != 0 {
        let ans = callee_thiscall!(2, u32, relocated(0x115dc18), w0, this, a2, a3);
        let sel = if ans == 0 {
            0xFF
        } else {
            let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
            let stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
            let base = unsafe { global::<u32>(0x115d988).read_unaligned() };
            let entry = unsafe {
                (base
                    .wrapping_add(bank.wrapping_mul(0x6f40))
                    .wrapping_add(0x6f10) as *const u32)
                    .read_unaligned()
            };
            ans.wrapping_sub(entry).wrapping_div(stride) & 0xFF
        };
        unsafe {
            ((this + 0x48) as *mut u8).write_unaligned(sel as u8);
        }
    }
    let w3 = unsafe { ((info + 12) as *const u32).read_unaligned() };
    let a3 = callee_thiscall!(3, u32, this.wrapping_add(0xb8), w3);
    if unsafe { ((this + 0xde) as *const u8).read_unaligned() } == 0 {
        // Original clears only AL: the member answer's high bytes survive.
        return a3 & 0xFFFFFF00;
    }
    let vtable = unsafe { (this as *const u32).read_unaligned() };
    let target = unsafe { ((vtable + 0x10) as *const u32).read_unaligned() };
    let resolve: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    let w1 = unsafe { ((info + 4) as *const u32).read_unaligned() };
    let r1 = resolve(this, w1);
    unsafe {
        ((this + 0xb0) as *mut u32).write_unaligned(r1);
    }
    let w2 = unsafe { ((info + 8) as *const u32).read_unaligned() };
    let r2 = resolve(this, w2);
    unsafe {
        ((this + 0xb4) as *mut u32).write_unaligned(r2);
    }
    // Original writes only AL (0 or 1): the second answer's high bytes
    // survive in both directions.
    if r2 == 0 || r1 == 0 {
        return r2 & 0xFFFFFF00;
    }
    (r2 & 0xFFFFFF00) | 1
});
