// original: 0x0059CC70 get_ready_global_object

/// Return and store the shared object when its first byte marks it ready.
///
/// The sole stdcall stack argument is an output pointer. A relocated global
/// supplies the object pointer; a null object or a zero first byte returns
/// false without touching the output, while a ready object is stored and
/// returns true.
lf_checker_rt::export!(stdcall, rw_0059cc70(output: u32) -> u32 {
    const OBJECT_POINTER_GLOBAL: u32 = 0x0110E6A0;

    unsafe {
        let object = lf_checker_rt::global::<u32>(OBJECT_POINTER_GLOBAL).read_unaligned();
        if object == 0 {
            return 0;
        }
        if (object as *const u8).read_unaligned() == 0 {
            return 0;
        }
        (output as *mut u32).write_unaligned(object);
        1
    }
});
