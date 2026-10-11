// original: 0x00D5AFD0 loader_release_request_handle (proposed)

/// Release the resource handle stored at offset `0x10` in the supplied
/// request record, then replace that field with the empty sentinel. The
/// explicit record pointer is the sole stack argument; the unused ECX value
/// is retained in the thiscall signature. The release helper's EAX result is
/// returned unchanged.
lf_checker_rt::export!(thiscall, rw_00D5AFD0(_owner: u32, request: u32) -> u32 {
    unsafe {
        const RESOURCE_HANDLE: u32 = 0x10;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        let handle = read_u32(request.wrapping_add(RESOURCE_HANDLE));
        let release_result = lf_checker_rt::callee_cdecl!(1, u32, handle);
        write_u32(request.wrapping_add(RESOURCE_HANDLE), u32::MAX);
        release_result
    }
});

