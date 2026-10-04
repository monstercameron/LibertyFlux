// original: 0x0099f770 audio_is_ped_speech_playing
/// Reports whether either ped speech slot is live, filling two out-words.
///
/// Uses the object at offset 0x60, falling back to the one at 0x64, and
/// returns zero when both are null. Otherwise each live object is probed:
/// both out-pointers must be set, the state byte at offset 0x3b must be 4,
/// and callee 1 must answer nonzero twice in a row; the second answer goes
/// through callee 2 into the second out-word and callee 3's answer into the
/// first. The return is one whenever an object was present. The original
/// clears the low byte of its incoming first-argument slot and lends callee 3
/// that slot address; the rewrite lends a spill copy holding the same masked
/// value instead, so the contract skips that call argument and the stack
/// check (the slot write is never read back).
fn fn_0099f770_block(obj: u32, base: u32, a0: u32, a1: u32) {
    unsafe {
        if a0 == 0 || a1 == 0 {
            return;
        }
        if *((obj.wrapping_add(0x3b)) as *const u8) != 4 {
            return;
        }
        if callee_thiscall!(1, u32, obj) == 0 {
            return;
        }
        let inner = callee_thiscall!(1, u32, obj);
        let v = callee_thiscall!(2, u32, inner, 0);
        *((a1) as *mut u32) = v;
        let mut slot = a0 & 0xffff_ff00;
        let w = callee_thiscall!(3, u32, base, &mut slot as *mut u32 as u32);
        *((a0) as *mut u32) = w;
    }
}
export!(thiscall, rw_0099f770(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let p = *((this.wrapping_add(0x60)) as *const u32);
        if p != 0 {
            let base = *((this.wrapping_add(0x60)) as *const u32);
            fn_0099f770_block(p, base, a0, a1);
            return 1;
        }
        let q = *((this.wrapping_add(0x64)) as *const u32);
        if q == 0 {
            return 0;
        }
        let base = *((this.wrapping_add(0x64)) as *const u32);
        fn_0099f770_block(q, base, a0, a1);
        1
    }
});
