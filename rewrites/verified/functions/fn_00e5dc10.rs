// original: 0x00e5dc10 net_slot_alloc_register_dc10
/// Allocate slot five, publish its handle and version, then register.
///
/// Requests slot five from the allocator, stores the returned handle in
/// its dedicated global word pair, tags the slot version, then calls the
/// central registrar with its constant address argument and returns the
/// registrar answer. Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dc10() -> u32 {
    let handle: u32 = lf_checker_rt::callee_stdcall!(2, u32, 5);
    unsafe {
        *lf_checker_rt::global::<u32>(0x019D2F18) = handle;
        *lf_checker_rt::global::<u16>(0x019D2F1E) = 5;
    }
    lf_checker_rt::callee_cdecl!(3, u32, lf_checker_rt::relocated(0x00E6EC60))
});
