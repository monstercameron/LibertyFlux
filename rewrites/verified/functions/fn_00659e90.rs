// original: 0x00659E90 stream_refresh_entry_state

/// Ensure the collection has a work record, asking TLS slot zero's manager to
/// allocate one when `this + 0x24` is null. A new record is marked active and
/// its handle and auxiliary dwords are cleared. Rebuild the collection's
/// child-byte flags, release any prior handle through the manager vtable,
/// clear the record fields, and then run the lookup/update pair once per
/// collection entry. The count at `this + 4` and entry-pointer array at
/// `this + 0` are unsigned 16-bit and pointer fields respectively; return the
/// count as a dword.
lf_checker_rt::export!(thiscall, rw_00659E90(this: u32) -> u32 {
    unsafe {
        const ENTRY_ARRAY: u32 = 0x00;
        const ENTRY_COUNT: u32 = 0x04;
        const WORK_RECORD: u32 = 0x24;
        const CONTEXT_MANAGER: u32 = 0x08;
        const VTABLE_ALLOCATE: u32 = 0x08;
        const VTABLE_RELEASE: u32 = 0x0C;
        const RECORD_MARKER: u32 = 0x00;
        const RECORD_HANDLE: u32 = 0x04;
        const RECORD_AUX: u32 = 0x08;
        const REQUEST_BYTES: u32 = 0x0C;
        const REQUEST_KIND: u32 = 0x10;
        const BUILD_FLAGS: u32 = 5;
        const LOOKUP_ENTRY: u32 = 6;
        const UPDATE_ENTRY: u32 = 7;

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

        #[inline(always)]
        unsafe fn manager_from_tls() -> u32 {
            let context = lf_checker_rt::tls_slot(0);
            unsafe { read_u32(context.wrapping_add(CONTEXT_MANAGER)) }
        }

        let mut work_record = read_u32(this.wrapping_add(WORK_RECORD));
        if work_record == 0 {
            let manager = manager_from_tls();
            let vtable = read_u32(manager);
            let allocate_address = read_u32(vtable.wrapping_add(VTABLE_ALLOCATE));
            let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(allocate_address as usize);
            work_record = allocate(manager, REQUEST_BYTES, REQUEST_KIND, 0);
            if work_record != 0 {
                write_u32(work_record.wrapping_add(RECORD_HANDLE), 0);
                write_u32(work_record.wrapping_add(RECORD_AUX), 0);
                write_u8(work_record.wrapping_add(RECORD_MARKER), 1);
            }
            write_u32(this.wrapping_add(WORK_RECORD), work_record);
        }

        let _flags = lf_checker_rt::callee_thiscall!(BUILD_FLAGS, u32, this);
        work_record = read_u32(this.wrapping_add(WORK_RECORD));
        let old_handle = read_u32(work_record.wrapping_add(RECORD_HANDLE));
        if old_handle != 0 {
            let manager = manager_from_tls();
            let vtable = read_u32(manager);
            let release_address = read_u32(vtable.wrapping_add(VTABLE_RELEASE));
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(release_address as usize);
            let _released = release(manager, old_handle);
        }

        write_u32(work_record.wrapping_add(RECORD_HANDLE), 0);
        write_u32(work_record.wrapping_add(RECORD_AUX), 0);
        write_u8(work_record.wrapping_add(RECORD_MARKER), 1);

        let entry_count = read_u16(this.wrapping_add(ENTRY_COUNT));
        let entries = read_u32(this.wrapping_add(ENTRY_ARRAY));
        let mut index = 0u32;
        while index < entry_count {
            let entry = read_u32(entries.wrapping_add(index.wrapping_mul(4)));
            let key = read_u32(entry);
            let mut handle = lf_checker_rt::callee_cdecl!(LOOKUP_ENTRY, u32, key, 0);
            let mut callback_index = index;
            let handle_out = &mut handle as *mut u32 as u32;
            let index_out = &mut callback_index as *mut u32 as u32;
            let _updated = lf_checker_rt::callee_thiscall!(
                UPDATE_ENTRY,
                u32,
                work_record,
                handle_out,
                index_out,
            );
            index = callback_index.wrapping_add(1);
        }

        entry_count
    }
});
