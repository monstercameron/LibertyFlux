// original: 0x00CF1F50 CPlayStatBase::CPlayStatBase_2

/// Initialize a play-stat base object with two caller-supplied values.
///
/// `this` points to the object; `kind` and `value` are the two stack
/// arguments stored at offsets `+0x04` and `+0x08`. The constructor installs
/// the class vtable, clears `+0x0c` and `+0x30`, clears byte `+0x10`, returns
/// `this` in EAX, and removes both arguments using thiscall cleanup.
lf_checker_rt::export!(thiscall, rw_fn_00cf1f50(this: u32, kind: u32, value: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00ed_e574;
    const KIND: u32 = 0x04;
    const VALUE: u32 = 0x08;
    const CLEARED_WORDS: [u32; 2] = [0x0c, 0x30];
    unsafe {
        let object = this as *mut u8;
        object.add(KIND).cast::<u32>().write_unaligned(kind);
        object.add(VALUE).cast::<u32>().write_unaligned(value);
        object.cast::<u32>().write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        for offset in CLEARED_WORDS {
            object.add(offset as usize).cast::<u32>().write_unaligned(0);
        }
        object.add(0x10).write(0);
    }
    this
});
