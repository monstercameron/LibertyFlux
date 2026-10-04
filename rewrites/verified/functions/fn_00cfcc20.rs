// original: 0x00cfcc20 euphoria_ctor_subobj_tag
/// Constructor for an animation object with an embedded member.
///
/// Runs the shared base constructor, stamps this object's virtual table,
/// zeroes a link word, constructs the embedded member at offset 0x18,
/// then stores the tag parameter and a -1 marker word. Returns the
/// object pointer.
lf_rs85_rt::export!(thiscall, rw_00cfcc20(this: *mut u8, tag: u32) -> u32 {
    unsafe {
        lf_rs85_rt::callee_thiscall!(1, u32, this as u32);
        let w = |off: usize| this.add(off) as *mut u32;
        *w(0) = lf_rs85_rt::relocated(0x00EE0534);
        *w(0x14) = 0;
        lf_rs85_rt::callee_thiscall!(2, u32, this.add(0x18) as u32);
        *w(0x28) = tag;
        *w(0x24) = 0xFFFF_FFFF;
        this as u32
    }
});
