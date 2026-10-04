// original: 0x00c66010 CCutsceneObject::vf11
/// True (1) when the cutscene object is in state 2, else 0.
export!(thiscall, rw_c66010(this: u32) -> u32 {
    let state = unsafe { (this as *const u32).byte_add(0x314).read() };
    (state == 2) as u32
});
