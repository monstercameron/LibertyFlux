// original: 0x00C78400 phbound_lookup_and_dispatch (proposed)

/// Ask the allocator for an object using the second input word and a zero
/// secondary argument. When an object is returned, load its `+4` field,
/// follow that field's vtable, and invoke virtual slot `+0x20` with the third
/// input word. The first input word is unused. Return true in AL on either
/// allocation path.
lf_checker_rt::export!(cdecl, rw_phbound_lookup_and_dispatch(_unused: u32, key: u32, callback_arg: u32) -> u32 {
    unsafe {
        const ALLOCATE: u32 = 1;
        const OBJECT_FIELD: u32 = 4;
        const VTABLE: u32 = 0;
        const DISPATCH_SLOT: u32 = 0x20;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let object = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, key, 0);
        if object != 0 {
            let field = unsafe { read_u32(object + OBJECT_FIELD) };
            let vtable = unsafe { read_u32(field + VTABLE) };
            let dispatch = unsafe { read_u32(vtable + DISPATCH_SLOT) };
            let callback: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                core::mem::transmute(dispatch as usize)
            };
            let _ = callback(field, callback_arg);
        }
        1
    }
});
