// original: 0x00b24580 CPhysical::vf62
// s08_b24580 (CPhysical::vf62): add to health. thiscall/1 (amount: f32):
// *(this+0x1F0) += amount. Returns nothing.
export!(thiscall, rw_b24580(this: *mut u8, amount: f32) -> () {
    unsafe {
        let hp = this.add(0x1F0) as *mut f32;
        *hp += amount;
    }
});
