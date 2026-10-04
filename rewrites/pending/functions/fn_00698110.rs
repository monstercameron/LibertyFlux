// original: 0x00698110 rage::crCreatureComponentMover::vf2
/// Clear the three link words of a creature mover component.
///
/// Zeroes the dwords at offsets 4, 8 and 0xc of the object. Returns nothing;
/// the original leaves EAX untouched.
export!(thiscall, rw_00698110(this: u32) -> () {
    unsafe {
        let p = this as *mut u32;
        *p.add(3) = 0;
        *p.add(1) = 0;
        *p.add(2) = 0;
    }
});
