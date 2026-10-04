// original: 0x00962CF0 prepare_and_load_file
/// Copy the path, resolve it through the open helpers, and load the file.
///
/// Returns 0 unless every stage succeeds: the path must be non-null and
/// non-empty with a nonzero second argument; a copy (length plus 8) is
/// allocated and filled, extended with a 5-byte suffix when helper 3
/// declines it, and passed to the open pair (helpers 4 and 5). The lookup
/// (helper 8) is retried up to 10 times with a wait between attempts, then
/// after an optional ready wait (up to 100 waits while the ready flag is
/// set) tried once more under the second key (helper 10). The sized buffer
/// (helper 11 reports the size, aligned to 16) is allocated, formatted
/// (helper 14), deleted through the import slot, reopened (helper 16), and
/// read (helpers 17 and 18/19). Every failure frees what it allocated and
/// returns 0; the full path frees both buffers and returns 1.
///
/// The stack cookie checks run for real on the original side (they pass
/// self-consistently); the import-slot call is intercepted by name.
export!(cdecl, rw_00962CF0(path: u32, arg1: u32, arg2: u32) -> u8 {
    unsafe {
        if path == 0 || *(path as *const u8) == 0 || arg1 == 0 {
            return 0;
        }
        let mut len = 0u32;
        while *((path.wrapping_add(len)) as *const u8) != 0 {
            len = len.wrapping_add(1);
        }
        let size = len.wrapping_add(8);
        let ebp: u32 = callee_cdecl!(1, u32, size);
        if ebp == 0 {
            return 0;
        }
        callee_cdecl!(2, u32, ebp, 0, size);
        let mut i = 0u32;
        loop {
            let b = *((path.wrapping_add(i)) as *const u8);
            *((ebp.wrapping_add(i)) as *mut u8) = b;
            i = i.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        let done: u32 = callee_cdecl!(3, u32, ebp, relocated(0xE8A968));
        if done == 0 {
            let suffix = *(relocated(0xE8A984) as *const u32);
            let suffix_byte = *(relocated(0xE8A988) as *const u8);
            let mut end = 0u32;
            while *((ebp.wrapping_add(end)) as *const u8) != 0 {
                end = end.wrapping_add(1);
            }
            *((ebp.wrapping_add(end)) as *mut u32) = suffix;
            *((ebp.wrapping_add(end).wrapping_add(4)) as *mut u8) = suffix_byte;
        }
        callee_cdecl!(4, u32, arg2, 0);
        let first: u32 = callee_cdecl!(5, u32, path);
        if first == 0 {
            callee_cdecl!(20, u32, ebp);
            return 0;
        }
        callee_thiscall!(6, u32, arg1, first, 0x4B);
        callee_cdecl!(7, u32, first);
        let mut edi = arg2;
        let mut count = 0u32;
        let mut found = false;
        loop {
            callee_cdecl!(4, u32, edi, 0);
            edi = callee_cdecl!(8, u32, path, relocated(0xE8A9B8));
            if edi != 0 {
                found = true;
                break;
            }
            callee_cdecl!(9, u32, 0x64);
            edi = arg2;
            count = count.wrapping_add(1);
            if count >= 10 {
                break;
            }
        }
        if !found {
            if *(relocated(0x1037868) as *const u8) != 0 {
                let mut waits = 100u32;
                while waits > 0 {
                    callee_cdecl!(9, u32, 0x64);
                    waits -= 1;
                    if *(relocated(0x1037868) as *const u8) == 0 {
                        break;
                    }
                }
            }
            callee_cdecl!(4, u32, edi, 0);
            edi = callee_cdecl!(10, u32, path, relocated(0xE8A9E4));
            if edi == 0 {
                callee_cdecl!(20, u32, ebp);
                return 0;
            }
        }
        let size1: u32 = callee_thiscall!(11, u32, edi);
        let aligned = size1.wrapping_add(0xF) & 0xFFFF_FFF0;
        let buf: u32 = callee_cdecl!(12, u32, aligned);
        if buf == 0 {
            callee_cdecl!(20, u32, ebp);
            callee_cdecl!(7, u32, edi);
            return 0;
        }
        callee_cdecl!(2, u32, buf, 0, aligned);
        let size2: u32 = callee_thiscall!(11, u32, edi);
        callee_thiscall!(13, u32, edi, buf, size2);
        callee_cdecl!(7, u32, edi);
        let mut msg = [0u8; 64];
        callee_cdecl!(
            14,
            u32,
            msg.as_mut_ptr() as u32,
            relocated(0xE8AA30),
            relocated(0x11695D8),
            arg2,
            path
        );
        let mut target = [0u8; 64];
        callee_stdcall!(15, u32, target.as_mut_ptr() as u32);
        callee_cdecl!(4, u32, arg2, 0);
        let second: u32 = callee_cdecl!(16, u32, ebp);
        if second == 0 {
            callee_cdecl!(20, u32, ebp);
            callee_cdecl!(20, u32, buf);
            return 0;
        }
        let answer: u32 = callee_cdecl!(17, u32, buf, aligned);
        let mut answer_home = answer;
        callee_thiscall!(18, u32, second, &mut answer_home as *mut u32 as u32, 4);
        callee_thiscall!(19, u32, second, buf, aligned);
        callee_cdecl!(7, u32, second);
        callee_cdecl!(20, u32, ebp);
        callee_cdecl!(20, u32, buf);
        1
    }
});
