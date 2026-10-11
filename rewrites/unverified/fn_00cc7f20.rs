// original: 0x00CC7F20 fragInstGta::vf5

/// Builds a four-word feature result for the receiver's nested physics record.
/// It clears the first three words of the caller's output, then checks the
/// nested record's kind bits (`+0x28`) and type (`+0x1304`). For kind `0x80`
/// and type 3, it calls the record's virtual slot at `+0xec`, passing a
/// four-word stack scratch area, and copies the returned record's words at
/// `+0`, `+4`, `+8`, and `+0xc` to the output. The callee's stack pointer
/// argument is compared by a four-word snapshot; the contract sets the
/// otherwise-uninitialized scratch area to zero.
lf_checker_rt::export!(thiscall, rw_00cc7f20(this: u32, output: u32) -> () {
    const RECORD_POINTER: u32 = 0x0c;
    const RECORD_KIND: u32 = 0x28;
    const RECORD_TYPE: u32 = 0x1304;
    const KIND_MASK: u32 = 0x3c0;
    const REQUIRED_KIND: u32 = 0x80;
    const REQUIRED_TYPE: u32 = 3;
    const VTABLE_POINTER: u32 = 0;
    const VTABLE_METHOD: u32 = 0xec;
    const RESULT_WORDS: [u32; 4] = [0, 4, 8, 12];

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        write_u32(output, 0);
        write_u32(output.wrapping_add(4), 0);
        write_u32(output.wrapping_add(8), 0);

        let record = read_u32(this.wrapping_add(RECORD_POINTER));
        if record == 0 {
            return;
        }
        if read_u32(record.wrapping_add(RECORD_KIND)) & KIND_MASK != REQUIRED_KIND {
            return;
        }
        if read_u32(record.wrapping_add(RECORD_TYPE)) != REQUIRED_TYPE {
            return;
        }

        let vtable = read_u32(record.wrapping_add(VTABLE_POINTER));
        let method_address = read_u32(vtable.wrapping_add(VTABLE_METHOD));
        let method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(method_address);
        let mut scratch = [0u32; 4];
        let result = method(record, scratch.as_mut_ptr() as u32);
        for offset in RESULT_WORDS {
            write_u32(output.wrapping_add(offset), read_u32(result.wrapping_add(offset)));
        }
    }
});
