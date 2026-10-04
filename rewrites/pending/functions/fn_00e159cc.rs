// original: 0x00e159cc locale_strchr
/// Find a character in a string using the locale class table.
///
/// Initialises a four-word cursor on the stack through the thiscall/1 setup
/// routine (stubbed as id 1, which fills the cursor by script), then: a null
/// string takes the error path (error-slot routine id 2 records the code
/// `0x16`, report routine id 3 runs, null is returned); when the class
/// table's flag word is zero the search is delegated to the vector routine
/// (cdecl/2, stubbed as id 4); otherwise the string is scanned directly, with
/// bytes whose class entry has bit 2 set treated as two-byte lead bytes
/// matched as a big-endian pair against the full sought value, and other
/// bytes matched singly. A trailing null matches a sought zero. On every
/// path, when the cursor's flag byte is set, bit 1 of the consumer's status
/// word (`0x70` past the cursor's consumer pointer) is cleared. Returns the
/// address of the match or null.
export!(cdecl, rw_00e159cc(s: *const u8, ch: u32, table: u32) -> u32 {
    unsafe {
        const NULL_CODE: u32 = 0x16;
        const LEAD_MASK: u8 = 4;
        const TABLE_FLAG_OFF: usize = 8;
        const CLASS_OFF: usize = 0x19;
        const STATUS_OFF: usize = 0x70;
        let mut st = [0u32; 4];
        let stp = st.as_mut_ptr() as u32;
        let _: u32 = callee_thiscall!(1, u32, stp, table);
        if s.is_null() {
            let slot = callee_cdecl!(2, u32,) as *mut u32;
            *slot = NULL_CODE;
            let _: u32 = callee_cdecl!(3, u32,);
            if st[3] as u8 != 0 {
                let consumer = st[2] as *mut u32;
                *consumer.add(STATUS_OFF / 4) &= !2u32;
            }
            return 0;
        }
        let ctype = st[1] as *const u8;
        let mut found = 0u32;
        if *(ctype.add(TABLE_FLAG_OFF) as *const u32) == 0 {
            found = callee_cdecl!(4, u32, s as u32, ch);
        } else {
            let mut p = s;
            loop {
                let c = *p;
                if c == 0 {
                    if ch == 0 {
                        found = p as u32;
                    }
                    break;
                }
                let cls = *ctype.add(c as usize + CLASS_OFF);
                if cls & LEAD_MASK != 0 {
                    let c2 = *p.add(1);
                    if c2 == 0 {
                        break;
                    }
                    if (((c as u32) << 8) | c2 as u32) == ch {
                        found = p as u32;
                        break;
                    }
                    p = p.add(2);
                } else {
                    if c as u32 == ch {
                        found = p as u32;
                        break;
                    }
                    p = p.add(1);
                }
            }
        }
        if st[3] as u8 != 0 {
            let consumer = st[2] as *mut u32;
            *consumer.add(STATUS_OFF / 4) &= !2u32;
        }
        found
    }
});
