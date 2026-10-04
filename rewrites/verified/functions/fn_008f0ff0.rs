// original: 0x008f0ff0 pools_attach
/// Attach the object's six pools through the shared creator: a null handle
/// fails; otherwise each pool is probed, then four pools attach in order
/// with short-circuit on the first zero answer, the handle is released, and
/// the last answer's low byte is returned.
export!(thiscall, rw_008f0ff0(this: u32, a0: u32, a1: u32) -> u32 {
    let h = callee_thiscall!(
        1,
        u32,
        relocated(0x110C0A0),
        a0,
        relocated(0xE83728),
        0,
        1
    );
    if h == 0 {
        return 0;
    }
    let s7b8 = this.wrapping_add(0x7B8);
    let sf70 = this.wrapping_add(0xF70);
    let s32ac = this.wrapping_add(0x32AC);
    let s1ee0 = this.wrapping_add(0x1EE0);
    let s1728 = this.wrapping_add(0x1728);
    let _ = callee_thiscall!(2, u32, this);
    let _ = callee_thiscall!(2, u32, s7b8);
    let _ = callee_thiscall!(2, u32, sf70);
    let _ = callee_thiscall!(2, u32, s32ac);
    let _ = callee_thiscall!(2, u32, s1ee0);
    let _ = callee_thiscall!(2, u32, s1728);
    let mut bl = callee_thiscall!(3, u32, this, h, a1) as u8;
    if bl != 0 {
        bl = callee_thiscall!(4, u32, s7b8, h, a1) as u8;
        if bl != 0 {
            bl = callee_thiscall!(5, u32, sf70, h, a1) as u8;
            if bl != 0 {
                bl = callee_thiscall!(6, u32, s32ac, h, a1) as u8;
            }
        }
    }
    let _ = callee_thiscall!(7, u32, h);
    bl as u32
});
