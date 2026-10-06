// original: 0x00b06b10 init_scales_and_child
/// Publish unit scales, notify, and lazily attach the child object.
///
/// thiscall `(this)`: writes 1.0f to two global scale words, calls the
/// object's vtable slot at `+0x18` as thiscall `(this)`, and when the
/// child pointer at `+0x110` is already set returns that call's answer
/// with its low byte replaced by 1. Otherwise it creates the child via
/// helper 2 (thiscall, no stack args; the helper stores the new child
/// at `+0x110`, scripted as an out-param write), initialises it via
/// helper 3 (thiscall `(child)`), zeroes its words at `+0x270`/`+0x278`
/// and byte at `+0x27c`, sets `+0x280` to -1, then links `+0x270` to
/// `this+0x10` and `+0x278` to `this`, returning the child pointer with
/// its low byte replaced by 1.
export!(thiscall, rw_00b06b10(this: u32) -> u32 {
    const SCALE_A: u32 = 0x0103_2350;
    const SCALE_B: u32 = 0x0103_234C;
    const ONE: u32 = 0x3F80_0000;
    unsafe {
        *global::<u32>(SCALE_A) = ONE;
        *global::<u32>(SCALE_B) = ONE;
        let vtbl = (this as *const u32).read_unaligned();
        let tgt = ((vtbl + 0x18) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
        let a1 = f(this);
        if ((this + 0x110) as *const u32).read_unaligned() != 0 {
            return (a1 & 0xFFFF_FF00) | 1;
        }
        let _: u32 = callee_thiscall!(2, u32, this);
        let child = ((this + 0x110) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(3, u32, child);
        ((child + 0x27C) as *mut u8).write(0);
        ((child + 0x270) as *mut u32).write_unaligned(0);
        ((child + 0x278) as *mut u32).write_unaligned(0);
        ((child + 0x280) as *mut u32).write_unaligned(0xFFFF_FFFF);
        let c2 = ((this + 0x110) as *const u32).read_unaligned();
        ((c2 + 0x270) as *mut u32).write_unaligned(this.wrapping_add(0x10));
        let c3 = ((this + 0x110) as *const u32).read_unaligned();
        ((c3 + 0x278) as *mut u32).write_unaligned(this);
        (c3 & 0xFFFF_FF00) | 1
    }
});
