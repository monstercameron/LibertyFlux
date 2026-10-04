// original: 0x00b25c90 CPhysical::vf61
// s08_b25c90 (CPhysical::vf61): set health with an ignored spare argument.
// thiscall/2 (hp: f32, _spare: u32): *(this+0x1F0) = hp. Returns nothing.
export!(thiscall, rw_b25c90(this: *mut u8, hp: f32, _spare: u32) -> () {
    unsafe {
        *(this.add(0x1F0) as *mut f32) = hp;
    }
});
