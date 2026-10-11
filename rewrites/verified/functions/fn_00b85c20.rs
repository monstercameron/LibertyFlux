// original: 0x00B85C20 script_vm_initialize_six_record_block

/// Clear six fixed records in the receiver's native state block.
///
/// The receiver is in ECX. Each pass writes the record's sentinel, clears its
/// status word and adjacent dword, and zeros the six-word payload window. The
/// return is the address immediately after the payload window.
lf_checker_rt::export!(thiscall, rw_00b85c20(this: u32) -> u32 {
    unsafe {
        const RECORDS: u32 = 6;
        let base = this as *mut u8;
        core::ptr::write_unaligned(base.add(0x54) as *mut u32, u32::MAX);
        for index in 0..RECORDS {
            core::ptr::write_unaligned(base.add(index * 4) as *mut u32, u32::MAX);
            core::ptr::write_unaligned(base.add(0x18 + index * 4) as *mut u32, 0);
            core::ptr::write_unaligned(base.add(0x30 + index * 2) as *mut u16, 0);
            core::ptr::write_unaligned(base.add(0x3c + index * 4) as *mut u32, 0);
        }
        this.wrapping_add(0x54)
    }
});
