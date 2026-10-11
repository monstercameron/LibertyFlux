// original: 0x005FE120 streaming_query_item_state

/// Query one item through the service and store its byte result.
///
/// The receiver's result byte is at `+0x1f8`. The single stack argument points
/// to a request whose service object is at `+4`. A local word is passed first
/// to a helper, then to that object's virtual method at vtable offset `+8`,
/// and finally to the function stored in a relocated global table slot. The
/// first two stand-ins write the local word, so their call-time snapshots
/// verify the shared output buffer across all three calls. The third call's
/// AL result is stored at `+0x1f8` and returned. A final cookie check receives
/// the relocated process cookie in ECX. This uses thiscall with one stack
/// argument.
lf_checker_rt::export!(thiscall, rw_005fe120(this: u32, request: u32) -> u8 {
    unsafe {
        const RESULT_BYTE: u32 = 0x1f8;
        const REQUEST_SERVICE: u32 = 4;
        const SERVICE_VTABLE_SLOT: u32 = 8;
        const TABLE_GLOBAL_VA: u32 = 0x018b7448;
        const TABLE_MEMBER: u32 = 0x3bc;
        const TABLE_METHOD_SLOT: u32 = 0x0c;
        const COOKIE_GLOBAL_VA: u32 = 0x01057fb4;

        let mut local_result = [0u8; 8];
        let direct_output = (local_result.as_mut_ptr().add(1)) as u32;
        let virtual_output = local_result.as_mut_ptr() as u32;

        let _ = lf_checker_rt::callee_cdecl!(1, u32, direct_output, 0, 0x3f);

        let service =
            (request.wrapping_add(REQUEST_SERVICE) as *const u32).read_unaligned();
        let vtable = (service as *const u32).read_unaligned();
        let virtual_target =
            (vtable.wrapping_add(SERVICE_VTABLE_SLOT) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(virtual_target as usize);
        let _ = query(service, virtual_output, 0x40);

        let table_base = lf_checker_rt::global::<u32>(TABLE_GLOBAL_VA)
            .read_unaligned()
            .wrapping_add(TABLE_MEMBER);
        let table_target = (table_base.wrapping_add(TABLE_METHOD_SLOT) as *const u32)
            .read_unaligned();
        let query: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(table_target as usize);
        let result = query(table_base, virtual_output);
        (this.wrapping_add(RESULT_BYTE) as *mut u8).write(result);

        let cookie = lf_checker_rt::global::<u32>(COOKIE_GLOBAL_VA).read_unaligned();
        let _ = lf_checker_rt::callee_thiscall!(4, (), cookie);
        result
    }
});
