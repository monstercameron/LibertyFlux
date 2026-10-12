// original: 0x00CF2940 phMaterialMgrGta::phMaterialMgrGta

/// Initialize a material-manager object before its tables are allocated.
///
/// `this` points to the object. The constructor installs the relocated
/// material-manager vtable, clears its pointer fields at `+0x04`, `+0x08`,
/// `+0x0c`, `+0x14`, and `+0x18`, and clears the byte at `+0x13`. It returns
/// `this` in EAX and otherwise leaves the object unchanged.
lf_checker_rt::export!(thiscall, rw_fn_00cf2940(this: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00ed_e9c4;
    const CLEARED_WORDS: [u32; 5] = [0x04, 0x0c, 0x08, 0x14, 0x18];
    unsafe {
        let manager = this as *mut u8;
        manager.add(0x04).cast::<u32>().write_unaligned(0);
        manager.cast::<u32>().write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        for offset in CLEARED_WORDS.iter().copied().skip(1) {
            manager.add(offset as usize).cast::<u32>().write_unaligned(0);
        }
        manager.add(0x13).write(0);
    }
    this
});
