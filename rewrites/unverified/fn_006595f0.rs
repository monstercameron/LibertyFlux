// original: 0x006595F0 stream_rebuild_entry_links

/// Reinitialize a collection's work record, then pass each selected entry to
/// a scripted lookup and update callback. `this + 0` is the entry-pointer
/// array, `this + 4` is its unsigned 16-bit count, and `this + 0x24` receives
/// the allocated work record. The allocator is obtained from TLS slot zero's
/// manager vtable. The two preparation calls receive the explicit resource
/// word and the receiver; the per-entry callback receives the work record
/// and pointers to a handle word and an iteration word. Its iteration output
/// is incremented and compared as a signed 32-bit value against the unsigned
/// 16-bit entry count. The method returns `this` and pops its one stack word.
lf_checker_rt::export!(thiscall, rw_006595F0(this: u32, resource: u32) -> u32 {
    unsafe {
        const ENTRY_ARRAY: u32 = 0x00;
        const ENTRY_COUNT: u32 = 0x04;
        const WORK_RECORD: u32 = 0x24;
        const CONTEXT_MANAGER: u32 = 0x08;
        const VTABLE_ALLOCATE: u32 = 0x08;
        const VTABLE_RELEASE: u32 = 0x0c;
        const RECORD_MARKER: u32 = 0x00;
        const RECORD_HANDLE: u32 = 0x04;
        const RECORD_AUX: u32 = 0x08;
        const REQUEST_SIZE: u32 = 0x0c;
        const REQUEST_KIND: u32 = 0x10;
        const PREPARE_CALL: u32 = 1;
        const RESOURCE_CALL: u32 = 2;
        const LOOKUP_CALL: u32 = 5;
        const UPDATE_CALL: u32 = 6;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u16) as u32 }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u32) }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { core::ptr::write(address as *mut u8, value) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { core::ptr::write_unaligned(address as *mut u32, value) }
        }

        // Prepare the receiver, then initialize its resource subobject.
        let _prepared = lf_checker_rt::callee_thiscall!(
            PREPARE_CALL,
            u32,
            this,
            resource,
            this,
        );
        let _resource_setup = lf_checker_rt::callee_thiscall!(
            RESOURCE_CALL,
            u32,
            this.wrapping_add(8),
            resource,
            this,
        );

        let context = lf_checker_rt::tls_slot(0);
        let manager = read_u32(context.wrapping_add(CONTEXT_MANAGER));
        let vtable = read_u32(manager);
        let allocate_address = read_u32(vtable.wrapping_add(VTABLE_ALLOCATE));
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let work_record = allocate(manager, REQUEST_SIZE, REQUEST_KIND, 0);

        write_u32(this.wrapping_add(WORK_RECORD), work_record);
        write_u32(work_record.wrapping_add(RECORD_HANDLE), 0);
        write_u32(work_record.wrapping_add(RECORD_AUX), 0);
        write_u8(work_record.wrapping_add(RECORD_MARKER), 1);

        let previous_handle = read_u32(work_record.wrapping_add(RECORD_HANDLE));
        if previous_handle != 0 {
            let release_address = read_u32(vtable.wrapping_add(VTABLE_RELEASE));
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(release_address as usize);
            let _released = release(manager, previous_handle);
        }

        write_u32(work_record.wrapping_add(RECORD_HANDLE), 0);
        write_u32(work_record.wrapping_add(RECORD_AUX), 0);
        write_u8(work_record.wrapping_add(RECORD_MARKER), 1);

        let entry_count = read_u16(this.wrapping_add(ENTRY_COUNT));
        let entry_array = read_u32(this.wrapping_add(ENTRY_ARRAY));
        let mut index = 0u32;
        while index < entry_count {
            let entry = read_u32(entry_array.wrapping_add(index.wrapping_mul(4)));
            let item = read_u32(entry);
            let mut handle = lf_checker_rt::callee_cdecl!(LOOKUP_CALL, u32, item, 0);
            let mut callback_index = index;
            let handle_out = &mut handle as *mut u32 as u32;
            let index_out = &mut callback_index as *mut u32 as u32;
            let _updated = lf_checker_rt::callee_thiscall!(
                UPDATE_CALL,
                u32,
                work_record,
                handle_out,
                index_out,
            );
            index = callback_index.wrapping_add(1);
        }

        this
    }
});
