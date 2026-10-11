// original: 0x00B10290 heli_create_attached_resource

/// Create and attach the resource owned by this entity when both prerequisite
/// handles are present. The allocator receives a 0x30-byte request. On success
/// the constructor receives the new object, the first handle, this method's
/// argument and the second handle; its result is then passed to the release
/// helper with the first handle. Allocation failure still calls that release
/// helper with a null result. If either prerequisite handle is null, no helper
/// is called. This is an x86 thiscall method with one stack argument.
lf_checker_rt::export!(thiscall, rw_00b10290(this: u32, argument: u32) -> u32 {
    const FIRST_HANDLE: u32 = 0xD0;
    const SECOND_HANDLE: u32 = 0xD4;
    const OBJECT_BYTES: u32 = 0x30;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let first_handle = read_u32(this.wrapping_add(FIRST_HANDLE));
        let second_handle = read_u32(this.wrapping_add(SECOND_HANDLE));
        if first_handle == 0 || second_handle == 0 {
            return 0;
        }

        let new_resource = lf_checker_rt::callee_cdecl!(1, u32, OBJECT_BYTES);
        if new_resource == 0 {
            let _ = lf_checker_rt::callee_thiscall!(3, u32, first_handle, 0u32);
        } else {
            let initialized = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                new_resource,
                first_handle,
                argument,
                second_handle
            );
            let _ = lf_checker_rt::callee_thiscall!(3, u32, first_handle, initialized);
        }
        0
    }
});
