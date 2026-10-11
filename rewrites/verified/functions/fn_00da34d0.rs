// original: 0x00DA34D0 ped_attractor_cleanup_records (proposed)

/// Walk the attractor's 16-byte record range backwards. For each record, the
/// secondary object is released through a helper and, when its byte flag is
/// clear, through virtual slot zero with argument one. A non-null primary
/// pointer is then removed from the three member collections. The record
/// array itself is not changed here; the helpers own those effects.
///
/// `self_ptr` is a pointer to the merged `CPedAttractor` layout. Its record
/// begin and end pointers are at `+0x30` and `+0x34`; each record stores its
/// primary pointer at `+0`, secondary pointer at `+4`, and dispatch flag at
/// `+0x0c`. The entry uses thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00da34d0(self_ptr: u32) -> () {
    unsafe {
        const RECORDS_BEGIN: u32 = 0x30;
        const RECORDS_END: u32 = 0x34;
        const RECORD_STRIDE: u32 = 0x10;
        const PRIMARY_POINTER: u32 = 0x00;
        const SECONDARY_POINTER: u32 = 0x04;
        const DISPATCH_FLAG: u32 = 0x0c;
        const ENABLE_DISPATCH_WHEN_CLEAR: u8 = 0;
        const DISPATCH_ARGUMENT: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let begin = read_u32(self_ptr.wrapping_add(RECORDS_BEGIN));
        let end = read_u32(self_ptr.wrapping_add(RECORDS_END));
        let bytes = end.wrapping_sub(begin) as i32;
        let mut record_index = (bytes >> 4).wrapping_sub(1);

        while record_index >= 0 {
            let record = begin.wrapping_add((record_index as u32).wrapping_mul(RECORD_STRIDE));
            let secondary = read_u32(record.wrapping_add(SECONDARY_POINTER));
            if secondary != 0 {
                let _ = lf_checker_rt::callee_thiscall!(1, u32, self_ptr, secondary);
                if read_u8(record.wrapping_add(DISPATCH_FLAG)) == ENABLE_DISPATCH_WHEN_CLEAR {
                    let table = read_u32(secondary);
                    let target = read_u32(table);
                    let dispatch: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    let _ = dispatch(secondary, DISPATCH_ARGUMENT);
                }
            }

            let primary = read_u32(record.wrapping_add(PRIMARY_POINTER));
            if primary != 0 {
                let _ = lf_checker_rt::callee_thiscall!(2, u32, self_ptr, primary);
                let _ = lf_checker_rt::callee_thiscall!(3, u32, self_ptr, primary);
                let _ = lf_checker_rt::callee_thiscall!(4, u32, self_ptr, primary);
            }
            record_index -= 1;
        }
    }
});
