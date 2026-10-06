// original: 0x00a99b40 mat_name_in_list_b

/// Report whether the object's name is one of five other fixed names.
///
/// Same shape as `rw_00a99ae0`: fetches the name through the object's
/// vtable slot at `+0x10` and compares it against the five NUL-terminated
/// names laid 0x40 bytes apart from `0x0103e988`. Returns 1 on the first
/// equal name, else 0.
///
/// Original: 0x00A99B40 (stdcall, one stack word; one indirect callee).
lf_checker_rt::export!(stdcall, rw_00a99b40(arg: u32) -> u32 {
    unsafe {
        /// Name-fetch slot in the object vtable.
        const VT_NAME: u32 = 0x10;
        /// Fixed name list (file VA): five entries, 0x40 apart.
        const LIST: u32 = 0x0103e988;
        const COUNT: u32 = 5;
        const STRIDE: u32 = 0x40;

        /// Byte-wise NUL-terminated string equality.
        unsafe fn streq(a: u32, b: u32) -> bool {
            unsafe {
                let mut x = a;
                let mut y = b;
                loop {
                    let ca = (x as *const u8).read();
                    let cb = (y as *const u8).read();
                    if ca != cb {
                        return false;
                    }
                    if ca == 0 {
                        return true;
                    }
                    x = x.wrapping_add(1);
                    y = y.wrapping_add(1);
                }
            }
        }

        let vt = (arg as *const u32).read_unaligned();
        let slot = ((vt.wrapping_add(VT_NAME)) as *const u32).read_unaligned();
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let name = fetch(arg);
        let mut ent = lf_checker_rt::relocated(LIST);
        for _ in 0..COUNT {
            if streq(name, ent) {
                return 1;
            }
            ent = ent.wrapping_add(STRIDE);
        }
        0
    }
});
