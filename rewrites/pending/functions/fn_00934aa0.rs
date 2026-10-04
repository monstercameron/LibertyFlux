// original: 0x00934aa0 init_set
/// 0x00934AA0: initialize a set object: clear the tag words, run the two
/// sub-initializers, clear the tag bytes, then tail-jump to the shared
/// state routine (modelled as a forwarding call).
export!(thiscall, rw_00934aa0(this: *mut u8) -> u32 {
    unsafe {
        core::ptr::write_unaligned(this.add(0x164) as *mut u32, 0);
        core::ptr::write_unaligned(this.add(0x168) as *mut u16, 0);
        core::ptr::write_unaligned(this.add(0x16c) as *mut u32, 0xffff_ffff);
        core::ptr::write_unaligned(this.add(0x170) as *mut u32, 0xffff_ffff);
    }
    callee_thiscall!(1, u32, (this as u32).wrapping_add(0x158));
    callee_cdecl!(2, u32, relocated(0x1036e90));
    unsafe {
        core::ptr::write_unaligned(this.add(0x140) as *mut u32, 0);
        for off in [0usize, 0x40, 0x80, 0xc0, 0x100] {
            core::ptr::write(this.add(off), 0u8);
        }
    }
    callee_thiscall!(3, u32, this as u32)
});
