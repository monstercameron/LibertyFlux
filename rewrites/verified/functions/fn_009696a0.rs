// original: 0x009696a0 select_peak_slot (proposed)

/// Pick the strongest of eight measured slots and a follow-up slot.
///
/// `key` selects which measurement the helper fills, and `out_idx`, when
/// non-null, receives the follow-up choice. While a global enable flag is
/// clear the function reports -1 for both choices without measuring.
/// Otherwise it asks the measurement helper to fill eight scratch floats,
/// takes the largest value seen scanning low to high (ties keep the later
/// slot; values below zero and NaNs never win), then scans the three
/// slots after the winner, wrapping around, keeping the largest value at
/// or above zero and reporting the last such slot. The winning index is
/// the return value; when every slot reads below zero both choices are -1.
///
/// The comparisons are ordered greater-or-equal tests, matching the
/// original's conditional jumps: a NaN measurement loses every
/// comparison, including against another NaN. The stack-cookie check the
/// original performs on exit is preserved as the same outgoing call; its
/// cookie value depends on the caller's frame address and is not part of
/// the comparison.
///
/// Original: 0x009696a0 (stdcall, two stack words; integer result; one
/// measurement call filling eight floats plus the cookie check).
lf_checker_rt::export!(stdcall, rw_009696a0(key: u32, out_idx: u32) -> u32 {
    unsafe {
        const ENABLE_FLAG: u32 = 0x0103_79D0; // file VA of the enable byte
        const COOKIE: u32 = 0x0105_7FB4; // file VA of the stack-cookie source
        const MEASURE_CALLEE: u32 = 1;
        const COOKIE_CALLEE: u32 = 2;
        const SLOTS: usize = 8;
        const NONE: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn check_cookie(frame_addr: u32) {
            unsafe {
                let raw = lf_checker_rt::global::<u32>(COOKIE).read();
                // The original xors the cookie source with its own stack
                // pointer; the value is frame-dependent and uncompared,
                // so any live frame address carries the same meaning.
                let cookie = raw ^ frame_addr;
                // No stack arguments: the cookie travels in ECX exactly
                // like the original, and the stub compares no registers.
                lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, cookie);
            }
        }

        let enabled = lf_checker_rt::global::<u8>(ENABLE_FLAG).read();
        if enabled == 0 {
            if out_idx != 0 {
                (out_idx as *mut u32).write_unaligned(NONE);
            }
            let probe = 0u32;
            check_cookie(&probe as *const u32 as u32);
            return NONE;
        }

        let mut buf = [0.0f32; SLOTS];
        lf_checker_rt::callee_stdcall!(
            MEASURE_CALLEE,
            u32,
            key,
            buf.as_mut_ptr() as u32
        );

        let mut best = 0.0f32;
        let mut idx: i32 = -1;
        if buf[0] >= 0.0 {
            best = buf[0];
            idx = 0;
        }
        let mut i = 1usize;
        while i < 7 {
            if buf[i] >= best {
                best = buf[i];
                idx = i as i32;
            }
            i += 1;
        }
        // The last slot can take the lead without moving the maximum.
        if buf[7] >= best {
            idx = 7;
        } else if idx < 0 {
            if out_idx != 0 {
                (out_idx as *mut u32).write_unaligned(NONE);
            }
            check_cookie(buf.as_mut_ptr() as u32);
            return NONE;
        }

        // Follow-up scan over the three slots after the winner.
        let mut run = 0.0f32;
        let mut sel: i32 = -1;
        let j = (idx + 3) % 8;
        if buf[j as usize] >= 0.0 {
            run = buf[j as usize];
            sel = j;
        }
        let k = (idx + 4) % 8;
        if buf[k as usize] >= run {
            run = buf[k as usize];
            sel = k;
        }
        let m = (idx + 5) % 8;
        if buf[m as usize] >= run {
            sel = m;
        }

        if out_idx != 0 {
            (out_idx as *mut u32).write_unaligned(sel as u32);
        }
        check_cookie(buf.as_mut_ptr() as u32);
        idx as u32
    }
});
