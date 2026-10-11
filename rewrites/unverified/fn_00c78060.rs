// original: 0x00C78060 pg_dictionary_phbound_destroy (proposed)

/// Destroy the dictionary's owned entries. It stores the class marker at
/// `this+0`, decrements the referenced object's count at `object+0x0c` and
/// calls its virtual slot zero when that count reaches zero. It then clears
/// `this+8`, invokes the child cleanup helper once for each 16-bit entry,
/// conditionally releases the child array and data buffer, and forwards
/// `this` to the final thiscall helper. The helper's return value is the
/// function's return value.
lf_checker_rt::export!(thiscall, rw_pg_dictionary_phbound_destroy(this: u32) -> u32 {
    unsafe {
        const CLASS_MARKER: u32 = 0x00ed_43d4;
        const OBJECT_PTR: u32 = 0x08;
        const OBJECT_REFS: u32 = 0x0c;
        const OBJECT_VTABLE: u32 = 0;
        const CHILD_ARRAY: u32 = 0x18;
        const CHILD_COUNT: u32 = 0x1c;
        const FREE_DATA_FLAG: u32 = 0x16;
        const FREE_ARRAY_FLAG: u32 = 0x1e;
        const DATA_BUFFER: u32 = 0x10;
        const CLEAN_CHILD: u32 = 2;
        const FREE_BUFFER: u32 = 3;
        const FINALIZE: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        unsafe { write_u32(this, CLASS_MARKER) };
        let object = unsafe { read_u32(this + OBJECT_PTR) };
        if object != 0 {
            let remaining = unsafe { read_u32(object + OBJECT_REFS) }.wrapping_sub(1);
            unsafe { write_u32(object + OBJECT_REFS, remaining) };
            if remaining == 0 {
                let vtable = unsafe { read_u32(object + OBJECT_VTABLE) };
                let release = unsafe { read_u32(vtable) };
                let callback: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                    core::mem::transmute(release as usize)
                };
                let _ = callback(object, 1);
            }
        }

        let children = unsafe { read_u32(this + CHILD_ARRAY) };
        let child_count = unsafe { read_u16(this + CHILD_COUNT) } as u32;
        unsafe { write_u32(this + OBJECT_PTR, 0) };
        for index in 0..child_count {
            let child = unsafe { read_u32(children.wrapping_add(index.wrapping_mul(4))) };
            let _ = lf_checker_rt::callee_thiscall!(CLEAN_CHILD, u32, child, 1);
        }

        if unsafe { read_u16(this + FREE_ARRAY_FLAG) } != 0 {
            let _ = lf_checker_rt::callee_cdecl!(FREE_BUFFER, u32, children);
        }
        if unsafe { read_u16(this + FREE_DATA_FLAG) } != 0 {
            let data = unsafe { read_u32(this + DATA_BUFFER) };
            let _ = lf_checker_rt::callee_cdecl!(FREE_BUFFER, u32, data);
        }
        lf_checker_rt::callee_thiscall!(FINALIZE, u32, this)
    }
});
