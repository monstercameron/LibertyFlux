// original: 0x009D00C0 dxut_error_dialog (proposed)

/// Format a Direct3D error message and show it in a message box.
///
/// Selects a message for the status `code` (only `0x80040901-0x80040908` and
/// `0x8004090A` name one; anything else, including `0x80040909`, takes the
/// default with no message and no dialog): code 1 picks between two messages
/// through callees 1 and 2, code 2 through the system-metrics callee 3, the
/// rest map to one message each. The message is copied into a 512-byte buffer
/// with forced NUL termination. The numeric code is stored at `+0x300` of the
/// state object (callee 4), and when the message flag and the byte at
/// `+0x2f2` are both set the buffer is shown with callee 7's window handle
/// and the application caption through the message-box callee 8. Every step
/// is wrapped in the flag-gated trace-hook pair (callee 5) shared with the
/// neighbouring reset routines. The entry cookie is re-checked (callee 9)
/// before returning; the return value is the dialog result on the dialog
/// path and the last hook/state-fetch value otherwise.
///
/// Only the empty-title path is proven: the contract pins the title lookup
/// (callee 6) to an empty string, so the stack-allocating converter path
/// never runs on either side (recorded in `narrowed`), and only the first 32
/// bytes of the message are observed (the checker's per-callee snapshot cap).
///
/// Original: 0x009D00C0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009D00C0(code: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x0103AD58;
        const HOOK_PRE_SLOT: u32 = 0x00E731CC;
        const HOOK_POST_SLOT: u32 = 0x00E731C8;
        const HOOK_ARG: u32 = 0x0129588C;
        const COOKIE: u32 = 0x01057FB4;
        const CAPTION: u32 = 0x00E96064;
        const MSG_BUF: usize = 512;
        const CODE_AT: u32 = 0x300;
        const SHOW_AT: u32 = 0x2f2;
        const MSG_PRESENT: u32 = 0x00E95D00;
        const MSG_ABSENT: u32 = 0x00E95DA0;
        const MSG_REMOTE: u32 = 0x00E95E7C;
        const MSG_NO_DEV: u32 = 0x00E95EAC;
        const MSG_NO_MEDIA: u32 = 0x00E95EDC;
        const MSG_REFCOUNT: u32 = 0x00E95F58;
        const MSG_CREATE: u32 = 0x00E95FB4;
        const MSG_RESET: u32 = 0x00E95FDC;
        const MSG_CREATE_CB: u32 = 0x00E95F1C;
        const MSG_RESET_CB: u32 = 0x00E96004;
        const MSG_REMOVED: u32 = 0x00E96040;
        const METRIC: u32 = 0x1000;
        const BOX_TYPE: u32 = 0x10;
        const PICK_A: u32 = 1;
        const PICK_B: u32 = 2;
        const SYS_METRIC: u32 = 3;
        const GET_OBJ: u32 = 4;
        #[allow(dead_code)]
        const HOOK: u32 = 5;
        const GET_TITLE: u32 = 6;
        const GET_HWND: u32 = 7;
        const MSGBOX: u32 = 8;
        const COOKIE_CHECK: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32h(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8h(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32h(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn flag() -> u8 {
            unsafe { (lf_checker_rt::relocated(FLAG) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn hook(ptr: u32, arg: u32) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(ptr as usize);
                f(arg)
            }
        }
        /// Bounded copy with forced NUL termination: up to 512 bytes from the
        /// image string at `src`, stopping at its NUL; a full buffer keeps 511
        /// bytes plus the terminator.
        unsafe fn copy_msg(dst: *mut u8, src: u32) {
            unsafe {
                let s = lf_checker_rt::relocated(src);
                let mut i = 0usize;
                while i < MSG_BUF {
                    let b = ((s + i as u32) as *const u8).read();
                    if b == 0 {
                        break;
                    }
                    dst.add(i).write(b);
                    i += 1;
                }
                if i == MSG_BUF {
                    dst.add(MSG_BUF - 1).write(0);
                } else {
                    dst.add(i).write(0);
                }
            }
        }

        let mut buf = [0u8; MSG_BUF];
        let dst = buf.as_mut_ptr();
        let mut show = true;
        let err: u32 = match code {
            0x80040901 => {
                if (lf_checker_rt::callee_cdecl!(PICK_A, u32,) as u8) != 0 {
                    if (lf_checker_rt::callee_cdecl!(PICK_B, u32,) as u8) == 0 {
                        copy_msg(dst, MSG_PRESENT);
                    } else {
                        copy_msg(dst, MSG_ABSENT);
                    }
                } else {
                    copy_msg(dst, MSG_ABSENT);
                }
                2
            }
            0x80040902 => {
                if lf_checker_rt::callee_stdcall!(SYS_METRIC, u32, METRIC) != 0 {
                    copy_msg(dst, MSG_REMOTE);
                } else {
                    copy_msg(dst, MSG_NO_DEV);
                }
                3
            }
            0x80040903 => {
                copy_msg(dst, MSG_NO_MEDIA);
                4
            }
            0x80040904 => {
                copy_msg(dst, MSG_REFCOUNT);
                5
            }
            0x80040905 => {
                copy_msg(dst, MSG_CREATE);
                6
            }
            0x80040906 => {
                copy_msg(dst, MSG_RESET);
                7
            }
            0x80040907 => {
                copy_msg(dst, MSG_CREATE_CB);
                8
            }
            0x80040908 => {
                copy_msg(dst, MSG_RESET_CB);
                9
            }
            0x8004090A => {
                copy_msg(dst, MSG_REMOVED);
                11
            }
            _ => {
                show = false;
                1
            }
        };
        let hook_arg = lf_checker_rt::relocated(HOOK_ARG);
        let caption = lf_checker_rt::relocated(CAPTION);
        // Store the code.
        let obj = lf_checker_rt::callee_cdecl!(GET_OBJ, u32,);
        let hook_pre = rd32(HOOK_PRE_SLOT);
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        wr32h(obj + CODE_AT, err);
        let hook_post = rd32(HOOK_POST_SLOT);
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        // Read the dialog gate.
        let obj = lf_checker_rt::callee_cdecl!(GET_OBJ, u32,);
        let mut ret = obj;
        if flag() != 0 {
            ret = hook(hook_pre, hook_arg);
        }
        let gate = rd8h(obj + SHOW_AT);
        if flag() != 0 {
            ret = hook(hook_post, hook_arg);
        }
        if show && gate != 0 {
            let title = lf_checker_rt::callee_cdecl!(GET_TITLE, u32,);
            if (title as *const u16).read_unaligned() == 0 {
                let hwnd = lf_checker_rt::callee_cdecl!(GET_HWND, u32, dst as u32, caption, BOX_TYPE);
                ret = lf_checker_rt::callee_stdcall!(MSGBOX, u32, hwnd, dst as u32, caption, BOX_TYPE);
            }
            // A non-empty title would take the stack-allocating converter
            // path; the contract pins the title empty, so this never runs.
        }
        let frame = &ret as *const u32 as u32;
        lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, rd32(COOKIE) ^ frame);
        ret
    }
});
