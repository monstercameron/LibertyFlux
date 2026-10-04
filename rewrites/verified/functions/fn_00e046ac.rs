// original: 0x00e046ac crt_heap wrapper (malloc/free/new/delete?)_4
/// Returns the heap block size for a pointer, or -1 for a null pointer.
///
/// For null, records error 0x16 through the error-slot callee, invokes
/// the error-report callee and returns -1; otherwise returns the block
/// size from the heap handle at 0x17AC2B4.
export!(cdecl, rw_00e046ac(ptr: u32) -> u32 {
    unsafe {
        if ptr == 0 {
            let slot = callee_cdecl!(1, u32,);
            *(slot as *mut u32) = 0x16;
            callee_cdecl!(2, u32,);
            return 0xFFFFFFFF;
        }
        let heap = *global::<u32>(0x17AC2B4);
        callee_stdcall!(3, u32, heap, 0, ptr)
    }
});
