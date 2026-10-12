// original: 0x00DFC344 __atof_l

/// Acquires the requested locale, skips leading locale whitespace, and asks
/// the locale parser helper for a double. A null input records EINVAL, calls
/// the invalid-parameter handler, and returns positive zero. If locale setup
/// marks a lock as acquired, the routine clears its in-use bit before return.
/// The parsed result is returned bit-for-bit through the x87 ST0 channel.
lf_checker_rt::export!(cdecl, rw_00dfc344(input: u32, locale: u32) -> f64 {
    unsafe {
        let mut context = [0u32; 11];
        let _ = lf_checker_rt::callee_thiscall!(1, u32, context.as_mut_ptr() as u32, locale);

        let value = if input == 0 {
            const EINVAL: u32 = 22;
            let errno_slot = lf_checker_rt::callee_cdecl!(2, u32,) as *mut u32;
            errno_slot.write(EINVAL);
            let _ = lf_checker_rt::callee_cdecl!(3, u32,);
            0.0f64
        } else {
            let locale_object = context[0];
            let mut text = input;
            let locale_kind = (locale_object.wrapping_add(0x74) as *const u32).read();

            if locale_kind as i32 > 1 {
                loop {
                    let byte = (text as *const u8).read() as u32;
                    let is_space = lf_checker_rt::callee_cdecl!(
                        4,
                        u32,
                        byte,
                        8u32,
                        context.as_ptr() as u32
                    );
                    if is_space == 0 {
                        break;
                    }
                    text = text.wrapping_add(1);
                }
            } else {
                let table = (locale_object.wrapping_add(0x90) as *const u32).read();
                loop {
                    let byte = (text as *const u8).read() as u32;
                    let entry = (table.wrapping_add(byte.wrapping_mul(2)) as *const u16).read();
                    if entry & 8 == 0 {
                        break;
                    }
                    text = text.wrapping_add(1);
                }
            }

            let mut end = 0u32;
            let parsed = lf_checker_rt::callee_cdecl!(
                5,
                u32,
                &mut end as *mut u32 as u32,
                text,
                context.as_ptr() as u32
            );
            (parsed.wrapping_add(0x10) as *const f64).read()
        };

        if context[3] & 0xff != 0 {
            let flags = context[2].wrapping_add(0x70) as *mut u32;
            flags.write(flags.read() & !2);
        }

        // The original's final call is its stack-cookie check. The rewrite
        // cannot reproduce the frame-derived cookie value in safe Rust.
        let _ = lf_checker_rt::callee_thiscall!(6, u32, 0);
        value
    }
});
