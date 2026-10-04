// original: 0x00c66020 CCutsceneObject::vf9
/// True (1) when the cutscene object is in state 0, else 0.
export!(thiscall, rw_c66020(this: u32) -> u32 {
    let state = unsafe { (this as *const u32).byte_add(0x314).read() };
    (state == 0) as u32
});
