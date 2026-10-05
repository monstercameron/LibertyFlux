// original: 0x008A9490 rage::audEffect::vf2 (merged symbol)

/// Refresh the effect's derived voice pointer, then tail-call the voice.
///
/// When `this+0x20` is nonzero and the flag byte at `this+0x71` is set, the
/// voice at `this + (([this+0x2c] * 5 + 13) * 4)` is refreshed through callee
/// `0x885220` (cdecl: `this`, derived pointer). Afterwards the voice pointer
/// at `this+8` is tail-called through its vtable slot at `+8` when non-null
/// (a computed tail jump; the checker plants the stub in a fabricated
/// vtable). No meaningful return value: the direct path leaves whatever the
/// last callee returned in `eax`, the null path leaves entry `eax`. Original
/// is thiscall with no stack words (plain `ret`).
lf_checker_rt::export!(thiscall, rw_008A9490(this: u32) -> u32 {
    const REFRESH: u32 = 1;
    const READY: u32 = 0x20;
    const ENABLED: u32 = 0x71;
    const INDEX: u32 = 0x2c;
    const VOICE: u32 = 8;
    const VTABLE_SLOT: u32 = 8;
    unsafe {
        if ((this + READY) as *const u32).read_unaligned() != 0
            && ((this + ENABLED) as *const u8).read() != 0
        {
            let x = ((this + INDEX) as *const u32).read_unaligned();
            let derived = this.wrapping_add(x.wrapping_mul(5).wrapping_add(13).wrapping_mul(4));
            lf_checker_rt::callee_cdecl!(REFRESH, u32, this, derived);
        }
        let obj = ((this + VOICE) as *const u32).read_unaligned();
        if obj != 0 {
            let vt = (obj as *const u32).read_unaligned();
            let slot = ((vt + VTABLE_SLOT) as *const u32).read_unaligned() as usize;
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot);
            f(obj);
        }
    }
    0
});
