// original: 0x00CDAB00 CTaskComplexNM::~CTaskComplexNM (symbols)

/// Install the derived-task vtable, optionally dispatch slot zero on the object at `this+0x28` with argument one, then tail-call the base destructor. The indirect target comes from the object's first vtable slot; the function has no stack arguments and returns the base-call result.
lf_checker_rt::export!(thiscall, rw_00cdab00(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const OBJECT_VTABLE: u32 = 0x00edd6e4;
        let owned = read_word(this + 0x28);
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        if owned != 0 {
            let table = read_word(owned);
            let target = read_word(table);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let _ = release(owned, 1);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
