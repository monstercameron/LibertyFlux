// original: 0x00C78300 phbound_allocate_and_attach (proposed)

/// Request an object for `key` with a zero secondary allocator argument. If
/// allocation succeeds, pass its `+4` field to the initializer with one zero
/// argument, then invoke virtual slot `+0x10` on the allocated object with
/// two zero arguments. The first incoming stack word is unused. Return true
/// in AL whether or not allocation succeeds.
lf_checker_rt::export!(cdecl, rw_phbound_allocate_and_attach(_unused: u32, key: u32) -> u32 {
    unsafe {
        const ALLOCATE: u32 = 1;
        const INITIALIZE_FIELD: u32 = 2;
        const FIELD: u32 = 4;
        const VTABLE: u32 = 0;
        const ATTACH_METHOD: u32 = 0x10;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let object = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, key, 0);
        if object != 0 {
            let field = unsafe { read_u32(object + FIELD) };
            let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_FIELD, u32, field, 0);
            let vtable = unsafe { read_u32(object + VTABLE) };
            let attach = unsafe { read_u32(vtable + ATTACH_METHOD) };
            let callback: extern "thiscall" fn(u32, u32, u32) -> u32 = unsafe {
                core::mem::transmute(attach as usize)
            };
            let _ = callback(object, 0, 0);
        }
        1
    }
});
