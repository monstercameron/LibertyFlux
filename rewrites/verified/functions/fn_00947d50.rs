// original: 0x00947d50 reset_subobject_table
/// Rebuild a table of 26 subobjects and clear its flag block.
///
/// Runs the element worker over all 26 entries, clears the flag bytes and
/// word that follow the array, then runs the second-phase worker over all
/// 26 entries again. Returns the table pointer.
lf_checker_rt::export!(thiscall, rw_00947d50(this_ptr: u32) -> u32 {
    unsafe {
        let mut s = this_ptr;
        let mut i = 0;
        while i < 26 {
            lf_checker_rt::callee_thiscall!(1, u32, s);
            s = s.wrapping_add(0x58);
            i += 1;
        }
        let f0 = this_ptr.wrapping_add(0x8f0) as *mut u8;
        *f0 = (*f0) & 0xfe;
        let f1 = this_ptr.wrapping_add(0x8f4) as *mut u32;
        *f1 = (*f1) & 0xfff8_0000;
        let f2 = this_ptr.wrapping_add(0x8f8) as *mut u8;
        *f2 = (*f2) & 0xfc;
        core::ptr::write_unaligned(this_ptr.wrapping_add(0x8f9) as *mut u16, 0);
        *(this_ptr.wrapping_add(0x8fb) as *mut u8) = 0;
        s = this_ptr;
        i = 0;
        while i < 26 {
            lf_checker_rt::callee_thiscall!(2, u32, s);
            s = s.wrapping_add(0x58);
            i += 1;
        }
        this_ptr
    }
});
