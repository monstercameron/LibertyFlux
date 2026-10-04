// original: 0x00c62b90 anim_bind_anim
/// Animation binder: tags the player live and stores the animation.
///
/// Writes tag 1 at `+0x44` and the animation argument at `+0x40`, returning
/// the argument.
export!(thiscall, rw_00c62b90(this: u32, anim: u32) -> u32 {
    unsafe {
        *((this + 0x44) as *mut u16) = 1;
        *((this + 0x40) as *mut u32) = anim;
        anim
    }
});
