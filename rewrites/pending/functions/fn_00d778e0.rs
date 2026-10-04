// original: 0x00d778e0 init_node_3arg
/// Initialize a 3-argument node: mix the sequence counter, store fields.
///
/// Xor-mixes a global sequence counter into the tag word at offset 4
/// (masking the mix to 14 bits), bumps the counter, then stores one plain
/// argument and two dereferenced ones. Returns the node pointer.
export!(thiscall, rw_00d778e0(this_: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        *(this_ as *mut u32) = relocated(0xE7E048);
        let ctr = global::<u32>(0x10327a0);
        let tag = (this_ + 4) as *mut u32;
        *tag ^= (*tag ^ *ctr) & 0x3fff;
        *ctr = (*ctr).wrapping_add(1);
        *((this_ + 8) as *mut u32) = a;
        *(this_ as *mut u32) = relocated(0xEEC724);
        *((this_ + 0xc) as *mut u32) = *(b as *const u32);
        *((this_ + 0x10) as *mut u32) = *(c as *const u32);
        this_
    }
});
