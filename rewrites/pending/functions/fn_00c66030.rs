// original: 0x00c66030 CCutsceneObject::vf10
/// True (1) when the cutscene object is in state 1, else 0.
export!(thiscall, rw_c66030(this: u32) -> u32 {
    let state = unsafe { (this as *const u32).byte_add(0x314).read() };
    (state == 1) as u32
});
