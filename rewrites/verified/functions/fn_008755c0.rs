// original: 0x008755c0 anim_set_mode_bytes
/// Store the two mode bytes of an animation request.
///
/// Copies the low byte of each stack argument into offsets `0x20` and
/// `0x21` of the object. The exit value of EAX is leftover argument
/// residue, so the rewrite returns nothing meaningful.
export!(thiscall, rw_008755c0(this: u32, mode0: u32, mode1: u32) -> u32 {
    unsafe {
        let base = this as *mut u8;
        base.add(0x20).write(mode0 as u8);
        base.add(0x21).write(mode1 as u8);
        0
    }
});
