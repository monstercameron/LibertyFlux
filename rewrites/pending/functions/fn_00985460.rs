// original: 0x00985460 audEmitterAudioEntity::audEmitterAudioEntity
/// Original 0x00985460 `audEmitterAudioEntity::audEmitterAudioEntity`.
///
/// Constructor: runs the base constructor, stamps the vtable, builds the
/// sub-object at +8, constructs 160 voice records (stride 0xd0 from +0x30),
/// initialises 2048 header slots (stride 0x60 from +0x826c), zeroes the
/// counter pair, registers 255 queue entries, and clears the tail flags.
/// Returns `this`.
export!(thiscall, rw_00985460(this_: u32) -> u32 {
    callee_thiscall!(1, u32, this_);
    unsafe { (this_ as *mut u32).write(relocated(0x00E8E2CC)); }
    callee_thiscall!(2, u32, this_.wrapping_add(8));
    let mut v = this_.wrapping_add(0x30);
    for _ in 0..160u32 {
        callee_thiscall!(3, u32, v);
        v = v.wrapping_add(0xd0);
    }
    let mut e = this_.wrapping_add(0x826c);
    for _ in 0..2048u32 {
        unsafe {
            ((e + 0x28) as *mut u8).write(((e + 0x28) as *const u8).read() & 0xfc);
            ((e - 0xc) as *mut u32).write(0xffff_ffff);
            (e as *mut u32).write(0);
            ((e + 4) as *mut u32).write(0);
            ((e - 4) as *mut u32).write(0);
            ((e + 0x26) as *mut u16).write(0xffff);
            ((e + 0xc) as *mut u32).write(0);
            ((e + 0x10) as *mut u32).write(0xffff_ffff);
        }
        e = e.wrapping_add(0x60);
    }
    unsafe {
        ((this_ + 0x38240) as *mut u32).write(0);
        ((this_ + 0x38244) as *mut u16).write(0);
    }
    let mut q = this_.wrapping_add(0x3a24c);
    for _ in 0..255u32 {
        callee_stdcall!(4, u32, q, 8, 0x20, relocated(0x00985530));
        unsafe { ((q + 0x100) as *mut u16).write(0); }
        q = q.wrapping_add(0x104);
    }
    unsafe {
        ((this_ + 0x8230) as *mut u32).write(0);
        ((this_ + 0x4a552) as *mut u8).write(0);
    }
    this_
});
