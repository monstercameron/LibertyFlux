// original: 0x009B6C70 create_registered_view (proposed)
/// Allocate a view object, register it, and return its result field.
///
/// Allocates the fixed-size object, runs its constructor, clears the three
/// header words, installs the table address, masks the flag word, stores the
/// object in the registry slot and writes the object's result field at `out`.
/// A failed allocation faults reading the result field, like the original.
/// stdcall, one out-pointer.
lf_checker_rt::export!(stdcall, rw_009B6C70(out: u32) -> u32 {
    unsafe {
        const SIZE: u32 = 0x570;
        const VTABLE: u32 = 0x00E93F7C;
        const FLAG_MASK: u32 = 0xF7FD9FE7;
        const FLAG_OFF: u32 = 0x40C;
        const RESULT_OFF: u32 = 0x53C;
        const REGISTRY: u32 = 0x0118D7F0;
        const REG_ARG: u32 = 0x10;
        let obj: u32 = lf_checker_rt::callee_cdecl!(1, u32, SIZE);
        if obj != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, obj);
            ((obj + 0x568) as *mut u32).write_unaligned(0);
            ((obj + 0x564) as *mut u32).write_unaligned(0);
            ((obj + 0x560) as *mut u32).write_unaligned(0);
            (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
            let f = ((obj + FLAG_OFF) as *const u32).read_unaligned();
            ((obj + FLAG_OFF) as *mut u32).write_unaligned(f & FLAG_MASK);
        }
        let slot: u32 = lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(REGISTRY), REG_ARG);
        (slot as *mut u32).write_unaligned(obj);
        lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(REGISTRY));
        let v = ((obj + RESULT_OFF) as *const u32).read_unaligned();
        (out as *mut u32).write_unaligned(v);
        out
    }
});
