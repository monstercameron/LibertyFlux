// original: 0x005d6c00 html_page_release
/// Release the optional page-owned objects and handles. Non-null object
/// fields use their virtual destructor; external handles use slot 3 of the
/// current thread's cleanup object. The inline list at offset 0x20 is always
/// passed to its cleanup helper. A present page-cache object at offset 0x0c
/// first triggers the static cache helper, then its virtual destructor.
/// Finally, the two process-global words at 0x019e8650 and 0x019e8654 are
/// cleared. `this` is in ECX; the function takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_005d6c00(this: u32) -> u32 {
    const PAGE_OBJECT_OFFSET: u32 = 0x00;
    const EXTERNAL_HANDLE_A_OFFSET: u32 = 0x08;
    const PAGE_CACHE_OBJECT_OFFSET: u32 = 0x0c;
    const EXTERNAL_HANDLE_B_OFFSET: u32 = 0x18;
    const INLINE_LIST_OFFSET: u32 = 0x20;
    const DESTRUCTOR_SLOT_OFFSET: u32 = 0x00;
    const THREAD_RELEASE_SLOT_OFFSET: u32 = 0x0c;
    const GLOBAL_HANDLE_ADDRESS: u32 = 0x019e_8650;
    const STATIC_PAGE_CACHE_ADDRESS: u32 = 0x0117_37d0;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn call_two_word_method(object: u32, slot_offset: u32, word: u32) -> u32 {
        let vtable = unsafe { read_u32(object) };
        let target = unsafe { read_u32(vtable.wrapping_add(slot_offset)) };
        let method: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        method(object, word)
    }
    #[inline(always)]
    unsafe fn call_thread_release(cleanup: u32, handle: u32, slot_offset: u32) -> u32 {
        unsafe { call_two_word_method(cleanup, slot_offset, handle) }
    }

    unsafe {
        let page_object = read_u32(this.wrapping_add(PAGE_OBJECT_OFFSET));
        if page_object != 0 {
            let _ = call_two_word_method(page_object, DESTRUCTOR_SLOT_OFFSET, 1);
            write_u32(this.wrapping_add(PAGE_OBJECT_OFFSET), 0);
        }

        let handle_a = read_u32(this.wrapping_add(EXTERNAL_HANDLE_A_OFFSET));
        if handle_a != 0 {
            let cleanup_environment = lf_checker_rt::tls_slot(0);
            let cleanup_object = read_u32(cleanup_environment.wrapping_add(8));
            let _ = call_thread_release(cleanup_object, handle_a, THREAD_RELEASE_SLOT_OFFSET);
            write_u32(this.wrapping_add(EXTERNAL_HANDLE_A_OFFSET), 0);
        }

        let cache_object = read_u32(this.wrapping_add(PAGE_CACHE_OBJECT_OFFSET));
        if cache_object != 0 {
            const STATIC_CACHE_HELPER: u32 = 4;
            let _ = lf_checker_rt::callee_thiscall!(
                STATIC_CACHE_HELPER,
                u32,
                lf_checker_rt::relocated(STATIC_PAGE_CACHE_ADDRESS),
                0
            );
            let _ = call_two_word_method(cache_object, DESTRUCTOR_SLOT_OFFSET, 1);
            write_u32(this.wrapping_add(PAGE_CACHE_OBJECT_OFFSET), 0);
        }

        let handle_b = read_u32(this.wrapping_add(EXTERNAL_HANDLE_B_OFFSET));
        if handle_b != 0 {
            let cleanup_environment = lf_checker_rt::tls_slot(0);
            let cleanup_object = read_u32(cleanup_environment.wrapping_add(8));
            let _ = call_thread_release(cleanup_object, handle_b, THREAD_RELEASE_SLOT_OFFSET);
        }
        write_u32(this.wrapping_add(EXTERNAL_HANDLE_B_OFFSET), 0);
        write_u32(this.wrapping_add(EXTERNAL_HANDLE_B_OFFSET + 4), 0);

        const INLINE_LIST_CLEANUP: u32 = 5;
        let _ = lf_checker_rt::callee_thiscall!(
            INLINE_LIST_CLEANUP,
            u32,
            this.wrapping_add(INLINE_LIST_OFFSET)
        );

        let global = lf_checker_rt::global::<u32>(GLOBAL_HANDLE_ADDRESS);
        let global_handle = global.read_unaligned();
        if global_handle != 0 {
            let cleanup_environment = lf_checker_rt::tls_slot(0);
            let cleanup_object = read_u32(cleanup_environment.wrapping_add(8));
            let _ = call_thread_release(cleanup_object, global_handle, THREAD_RELEASE_SLOT_OFFSET);
        }
        global.write_unaligned(0);
        global.add(1).write_unaligned(0);
        0
    }
});
