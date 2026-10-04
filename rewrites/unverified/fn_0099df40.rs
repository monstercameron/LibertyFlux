// original: 0x0099df40 voice_effect_apply
/// Applies a voice effect parameter through callee 1.
///
/// Uses the object at offset 0x60, falling back to the one at 0x64 and
/// returning silently when both are null. A marker byte of 0xff selects the
/// zero parameter; otherwise the parameter combines a global multiplier, the
/// marker byte, and one word from a global table indexed by a second byte
/// scaled by 0x6f40.
export!(thiscall, rw_0099df40(this: u32) -> () {
    unsafe {
        const MULT: u32 = 0x115d964;
        const TABLE: u32 = 0x115d988;
        const STRIDE: u32 = 0x6f40;
        const BIAS: u32 = 0x6f10;
        let mut p = *((this.wrapping_add(0x60)) as *const u32);
        if p == 0 {
            p = *((this.wrapping_add(0x64)) as *const u32);
            if p == 0 {
                return;
            }
        }
        let mark = *((p.wrapping_add(0x48)) as *const u8);
        if mark == 0xff {
            callee_thiscall!(1, u32, 0, 0);
            return;
        }
        let sel = *((p.wrapping_add(0x40)) as *const u8) as u32;
        let g1 = *global::<u32>(MULT);
        let g2 = *global::<u32>(TABLE);
        let cell = *((g2.wrapping_add(sel.wrapping_mul(STRIDE)).wrapping_add(BIAS)) as *const u32);
        let v = g1.wrapping_mul(mark as u32).wrapping_add(cell);
        callee_thiscall!(1, u32, v, 0);
    }
});
