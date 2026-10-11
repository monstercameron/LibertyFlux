// original: 0x00C77430 streaming_dispatch_entry
/// Dispatch a record through the streaming index and its interface method.
///
/// `index` selects a row from the global table, `record_root` points to a
/// three-level pointer chain, `dispatch_arg` is forwarded on the ordinary
/// record path, and `status_byte` is cleared before dispatch. The row's first
/// word is set to the nested record pointer. When the leaf kind is `0x0c`,
/// the function calls the method for each non-null child, passing
/// `status_byte`; otherwise it calls the row's second method object once,
/// passing `dispatch_arg`. The return value sets AL to one and retains the
/// upper 24 bits left by the last call, or by the leaf pointer when the child
/// count is zero. The index flag with bit `0x80` selects a null row and is a
/// faulting case before dispatch.
///
/// The original is cdecl with four stack words and a 32-bit boolean result in
/// AL. Its global table stores a row base, a flag-byte base and a row stride;
/// row offsets and leaf fields below are named from their observed use.
/// Game pointers in this executable are 32-bit words, while the Rust pointer
/// parameters use `usize` so the host type itself is pointer-width aware.
///lf_checker_rt::export!(cdecl, rw_00c77430(index: u32, record_root: usize, dispatch_arg: usize, status_byte: usize) -> u32 {
    unsafe {
        const STREAMING_TABLE_FILE_VA: u32 = 0x016E_C774;
        const TABLE_ROW_BASE: usize = 0x00;
        const TABLE_FLAG_BASE: usize = 0x04;
        const TABLE_ROW_STRIDE: usize = 0x0C;
        const ROW_RECORD: usize = 0x00;
        const ROW_METHODS: usize = 0x30;
        const ROOT_OBJECT: usize = 0x00;
        const OBJECT_NESTED_RECORD: usize = 0x08;
        const LEAF_KIND: usize = 0x04;
        const LEAF_CHILD_COUNT: usize = 0x92;
        const MULTI_CHILD_KIND: u8 = 0x0C;
        const INVALID_ROW_FLAG: u8 = 0x80;
        const VTABLE_DISPATCH_SLOT: usize = 0x20;
        const DISPATCH_CALLEE: u32 = 1;
        const TRUE_BYTE: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: usize) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: usize) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: usize) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: usize, value: u32) {
            unsafe { (address as *mut u32).write_volatile(value) }
        }
        #[inline(always)]
        unsafe fn write_u8(address: usize, value: u8) {
            unsafe { (address as *mut u8).write_volatile(value) }
        }
        #[inline(always)]
        fn true_result(eax: u32) -> u32 {
            (eax & 0xFFFF_FF00) | TRUE_BYTE
        }

        let table_pointer = unsafe {
            lf_checker_rt::global::<u32>(STREAMING_TABLE_FILE_VA).read_unaligned() as usize
        };
        let row_base = unsafe { read_u32(table_pointer.wrapping_add(TABLE_ROW_BASE)) as usize };
        let flag_base = unsafe { read_u32(table_pointer.wrapping_add(TABLE_FLAG_BASE)) as usize };
        let row_stride = unsafe { read_u32(table_pointer.wrapping_add(TABLE_ROW_STRIDE)) as usize };
        let row_is_invalid = unsafe { read_u8(flag_base.wrapping_add(index as usize)) & INVALID_ROW_FLAG != 0 };
        let row = if row_is_invalid {
            0usize
        } else {
            row_base.wrapping_add(row_stride.wrapping_mul(index as usize))
        };

        unsafe { write_u8(status_byte, 0); }

        let top_object = unsafe { read_u32(record_root.wrapping_add(ROOT_OBJECT)) as usize };
        let nested_record = unsafe { read_u32(top_object.wrapping_add(OBJECT_NESTED_RECORD)) as usize };
        unsafe { write_u32(row.wrapping_add(ROW_RECORD), nested_record as u32); }
        let leaf = unsafe { read_u32(nested_record.wrapping_add(OBJECT_NESTED_RECORD)) as usize };
        let kind = unsafe { read_u8(leaf.wrapping_add(LEAF_KIND)) };
        let methods = unsafe { read_u32(row.wrapping_add(ROW_METHODS)) as usize };
        let mut eax = leaf as u32;

        if kind == MULTI_CHILD_KIND {
            let child_count = unsafe { read_u16(leaf.wrapping_add(LEAF_CHILD_COUNT)) as u32 };
            let mut child_index = 0u32;
            while child_index < child_count {
                let mut_this = unsafe { read_u32(methods.wrapping_add((child_index as usize).wrapping_mul(4))) as usize };
                eax = methods as u32;
                if mut_this != 0 {
                    let interface = unsafe { read_u32(mut_this.wrapping_add(4)) as usize };
                    let vtable = unsafe { read_u32(interface) as usize };
                    let _slot_target = unsafe { read_u32(vtable.wrapping_add(VTABLE_DISPATCH_SLOT)) };
                    eax = lf_checker_rt::callee_thiscall!(
                        DISPATCH_CALLEE,
                        u32,
                        interface as u32,
                        status_byte as u32,
                    );
                }
                child_index = child_index.wrapping_add(1);
            }
        } else {
            let method_this = unsafe { read_u32(methods.wrapping_add(4)) as usize };
            let vtable = unsafe { read_u32(method_this) as usize };
            let _slot_target = unsafe { read_u32(vtable.wrapping_add(VTABLE_DISPATCH_SLOT)) };
            eax = lf_checker_rt::callee_thiscall!(
                DISPATCH_CALLEE,
                u32,
                method_this as u32,
                dispatch_arg as u32,
            );
        }
        true_result(eax)
    }
});
