// original: 0x00c2dc70 membership_test_register
/// Membership test with lazy registration of the owner.
///
/// Walks a four-level handle chain and tests one bit of a bitset indexed
/// by the low byte at +4. On a hit, sets the sticky bit, and when the
/// owner and its marker are present, registers (argument, this) with the
/// global registry. Returns 0 on the taken path, 1 on every early exit.
export!(thiscall, rw_00c2dc70(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let cl = *this.add(4);
        if (cl as i8) < 0 {
            return 1;
        }
        let flags = *(this.add(0x10) as *const u32);
        if ((flags >> 0x13) as u8) & 1 != 0 {
            return 0;
        }
        if arg == 0 {
            return 1;
        }
        let l1 = *((arg.wrapping_add(0xDC4)) as *const u32);
        if l1 == 0 {
            return 1;
        }
        let l2 = *((l1.wrapping_add(0x64)) as *const u32);
        if l2 == 0 {
            return 1;
        }
        let l3 = *((l2.wrapping_add(0x10C)) as *const u32);
        let words = *(l3 as *const u32);
        let idx = (cl as i8 as i32 >> 5) as u32;
        let bit = 1u32 << ((cl & 0x1F) as u32);
        if (*(words.wrapping_add(idx * 4) as *const u32)) & bit == 0 {
            return 1;
        }
        *(this.add(0x10) as *mut u32) = flags | 0x80000;
        let l4 = *((arg.wrapping_add(0x6C)) as *const u32);
        if l4 != 0 && *((l4.wrapping_add(0xE)) as *const u8) != 0 {
            callee_thiscall!(1, u32, relocated(0x18ECFB0), arg, this as u32);
        }
        0
    }
});
