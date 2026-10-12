// original: 0x00871F80 initialize_refcounted_embedded_object

/// Initialize the object's reference fields and its embedded subobject.
///
/// Install each relocated vtable address, clear the reference slots, and call
/// the setup hook from the referenced image slot with the embedded object at
/// `this + 0x60` in ECX. Store the address `this + 8` at `+0x6c`, then return
/// `this`. The hook's return value is ignored.
///
/// Original calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00871f80(this: u32) -> u32 {
    const OBJECT_VTABLE: u32 = 0x00fe7ee8;
    const EMBEDDED_OFFSET: u32 = 0x60;
    const EMBEDDED_VTABLE: u32 = 0x00fe7ef8;
    const SETUP_SLOT: u32 = 0x00fe7efc;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE));
        (this.wrapping_add(4) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x0c) as *mut u32).write_unaligned(0);
        let embedded = this.wrapping_add(EMBEDDED_OFFSET);
        (embedded as *mut u32).write_unaligned(lf_checker_rt::relocated(EMBEDDED_VTABLE));
        (embedded.wrapping_add(4) as *mut u32).write_unaligned(0);
        (embedded.wrapping_add(8) as *mut u32).write_unaligned(0);
        (embedded.wrapping_add(0x0c) as *mut u32).write_unaligned(0);
        let target = lf_checker_rt::global::<u32>(SETUP_SLOT).read_unaligned();
        let setup: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let _ignored = setup(embedded);
        (this.wrapping_add(0x6c) as *mut u32).write_unaligned(this.wrapping_add(8));
        this
    }
});
