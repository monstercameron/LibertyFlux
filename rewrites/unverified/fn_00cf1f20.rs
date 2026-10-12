// original: 0x00CF1F20 CPlayStatBase::CPlayStatBase

/// Initialize a play-stat base object with one category value.
///
/// `this` points to the object. The sole stack argument is stored at `+0x08`;
/// the vtable is installed at `+0x00`, fields `+0x04`, `+0x0c`, and `+0x30`
/// are cleared, and the byte at `+0x10` is cleared. The constructor returns
/// `this` in EAX and uses thiscall cleanup for its one stack argument.
lf_checker_rt::export!(thiscall, rw_fn_00cf1f20(this: u32, category: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00ed_e574;
    const CATEGORY: u32 = 0x08;
    const CLEARED_WORDS: [u32; 3] = [0x04, 0x0c, 0x30];
    unsafe {
        let object = this as *mut u8;
        object.add(CATEGORY).cast::<u32>().write_unaligned(category);
        object.cast::<u32>().write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        for offset in CLEARED_WORDS {
            object.add(offset as usize).cast::<u32>().write_unaligned(0);
        }
        object.add(0x10).write(0);
    }
    this
});
