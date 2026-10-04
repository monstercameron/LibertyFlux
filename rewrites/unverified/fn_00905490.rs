// original: 0x00905490 input_slots_init (proposed)

/// Initialise the input slot subsystem: set the mode flags, clear the slot
/// table, and build the two descriptor blocks the dispatcher consumes.
///
/// Sets four mode bytes, calls the allocator helper, zeroes all `0x5dc`
/// slot pointers in `TABLE`, calls the per-slot constructor helper, then
/// reads the selected variant (`SEL[index]` byte `+0x58`) into `VARIANT`,
/// clears five counter words and two more flags, and asks the descriptor
/// helper (thiscall, four stack arguments, out-block in ecx) twice for the
/// two 16-byte descriptors. Each descriptor gets its last word patched
/// with the constant `COOKIE`, and both are passed as eight words to the
/// dispatcher. Returns the
/// dispatcher's answer. Faults exactly when the original does: a wild
/// selector index faults on the entry load, a null or wild entry on the
/// variant read. Original: 0x00905490 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00905490() -> u32 {
    unsafe {
        const TABLE: u32 = 0x0118F6F8;
        const TABLE_LEN: usize = 0x5dc;
        const SEL: u32 = 0x0118E7F8;
        const SEL_INDEX: u32 = 0x0118EA08;
        const VARIANT: u32 = 0x010344D8;
        const VARIANT_OFF: u32 = 0x58;
        // Code addresses, relocated into the worker's mapping like the
        // original's own relocated pushes.
        let patch: u32 = lf_checker_rt::relocated(0x00430260);
        let desc_arg_a: u32 = lf_checker_rt::relocated(0x004016A0);
        let desc_arg_b: u32 = lf_checker_rt::relocated(0x00901BD0);

        ((lf_checker_rt::relocated(0x0118F4BC)) as *mut u8).write(1);
        ((lf_checker_rt::relocated(0x011E61C6)) as *mut u8).write(1);
        ((lf_checker_rt::relocated(0x011DB23D)) as *mut u8).write(0);
        ((lf_checker_rt::relocated(0x011DB23E)) as *mut u8).write(0);
        lf_checker_rt::callee_cdecl!(1, u32,);
        let tab = lf_checker_rt::relocated(TABLE) as *mut u32;
        for i in 0..TABLE_LEN {
            tab.add(i).write(0);
        }
        lf_checker_rt::callee_cdecl!(2, u32,);
        let idx = (lf_checker_rt::relocated(SEL_INDEX) as *const u32).read_unaligned();
        let ent = (lf_checker_rt::relocated(SEL).wrapping_add(idx.wrapping_mul(4))
            as *const u32)
            .read_unaligned();
        ((lf_checker_rt::relocated(0x0118F4BD)) as *mut u8).write(0);
        ((lf_checker_rt::relocated(0x0118F4BE)) as *mut u8).write(0);
        for a in [0x0118F4C8u32, 0x0118F4CC, 0x0118F4D4, 0x0118F4D8, 0x0118F4D0] {
            ((lf_checker_rt::relocated(a)) as *mut u32).write(0);
        }
        let v = (ent.wrapping_add(VARIANT_OFF) as *const u8).read();
        ((lf_checker_rt::relocated(VARIANT)) as *mut u8).write(v);

        let mut desc_b = [0u32; 4];
        let pb = core::hint::black_box(desc_b.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(3, u32, pb, 0u32, desc_arg_a, 0u32, 0u32);
        desc_b[3] = patch;
        let mut desc_a = [0u32; 4];
        let pa = core::hint::black_box(desc_a.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(3, u32, pa, 0u32, desc_arg_b, 0u32, 0u32);
        desc_a[3] = patch;
        lf_checker_rt::callee_cdecl!(
            4, u32, desc_b[0], desc_b[1], desc_b[2], desc_b[3], desc_a[0],
            desc_a[1], desc_a[2], desc_a[3]
        )
    }
});
