// original: 0x00b25230 CPhysical::vf60
// s08_b25230 (CPhysical::vf60): set health. thiscall/1 (hp: f32):
// *(this+0x1F0) = hp. Returns nothing.
export!(thiscall, rw_b25230(this: *mut u8, hp: f32) -> () {
    unsafe {
        *(this.add(0x1F0) as *mut f32) = hp;
    }
});
