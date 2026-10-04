// original: 0x009fb680 net_session_create (proposed)

/// Create a network session object from seven parameters and register it.
///
/// Takes seven cdecl stack arguments (`arg0`..`arg6`), all passed through as
/// values and never dereferenced. `arg3` selects the configuration source,
/// `arg4` and `arg5` are builder inputs, and `arg6` is the owner passed to
/// the final registration call. All working buffers live in the original's
/// frame and are zero-initialised; only call arguments, eight snapped words
/// of the build descriptor, and the return value are compared.
///
/// Sequence: callee 0 tests `arg3` (thiscall); a zero answer formats the
/// default configuration through callee 6 (cdecl, buffer, `DEFAULT_CFG`,
/// `CFG_LEN`). Otherwise callee 1 retests `arg3`: a nonzero answer resolves
/// through callee 2 and converts through callee 3 (cdecl, two buffers and
/// `CFG_LEN`), a zero answer resolves through callee 4 and converts through
/// callee 5. Callee 7 then validates a scratch buffer (cdecl, one pointer);
/// a zero answer clears the result word. Callee 10 builds the session handle
/// (thiscall) from a descriptor whose tag word is `DESC_TAG`, with callee 8,
/// 9, 11 and 12 preparing and checking the descriptor (thiscall/cdecl,
/// answers ignored). Callee 13 assembles the session (thiscall: descriptor
/// plus `arg0`, `arg1`, `arg2`, `arg4`, `arg5` and the handle); its first
/// word carries the resolver answer, or zero on the default path. Callee 14
/// opens the session (thiscall on the assembly result) and callee 15
/// registers it (thiscall on `arg6`: `REGION_PTR`, a global slot, four
/// buffers, two zero flags and the open result), whose answer is returned.
/// Callee 16 is the stack-cookie check (no arguments, registers preserved).
lf_checker_rt::export!(
    cdecl,
    rw_009fb680(arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, arg6: u32) -> u32 {
        unsafe {
            const CFG_LEN: u32 = 0x20;
            const DEFAULT_CFG: u32 = 0x00E9_960C;
            const DESC_TAG: u32 = 0x006E_6977;
            const REGION_PTR: u32 = 0x011A_4F80;
            const SHARED_SLOT: u32 = 0x011D_6FD4;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }

            // Shadow frame: the original's zeroed working buffers. Only the
            // build descriptor is ever observed (through callee 13's snapshot).
            let mut cfg_a = [0u32; 8];
            let mut cfg_b = [0u32; 8];
            let mut scratch = [0u32; 4];
            let mut desc = [0u32; 8];
            let mut reg_buf_a = [0u32; 8];
            let mut reg_buf_b = [0u32; 8];
            let mut reg_buf_c = [0u32; 8];
            let mut reg_buf_d = [0u32; 8];

            let default_cfg = lf_checker_rt::relocated(DEFAULT_CFG);
            let region_ptr = lf_checker_rt::relocated(REGION_PTR);
            let mut first_word = 0u32;
            let gate0: u32 = lf_checker_rt::callee_thiscall!(0, u32, arg3);
            if gate0 & 0xFF == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    6,
                    u32,
                    cfg_a.as_mut_ptr() as u32,
                    default_cfg,
                    CFG_LEN
                );
            } else {
                let gate1: u32 = lf_checker_rt::callee_thiscall!(1, u32, arg3);
                if gate1 & 0xFF != 0 {
                    first_word = lf_checker_rt::callee_thiscall!(2, u32, arg3);
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        3,
                        u32,
                        cfg_a.as_mut_ptr() as u32,
                        cfg_b.as_mut_ptr() as u32,
                        CFG_LEN
                    );
                } else {
                    first_word = lf_checker_rt::callee_thiscall!(4, u32, arg3);
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        5,
                        u32,
                        cfg_a.as_mut_ptr() as u32,
                        cfg_b.as_mut_ptr() as u32,
                        CFG_LEN
                    );
                }
            }
            let valid: u32 = lf_checker_rt::callee_cdecl!(7, u32, scratch.as_mut_ptr() as u32);
            if valid & 0xFF == 0 {
                scratch[0] = 0;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, desc.as_mut_ptr() as u32);
            let _: u32 = lf_checker_rt::callee_cdecl!(9, u32, desc.as_mut_ptr() as u32);
            desc[2] = DESC_TAG;
            let handle: u32 = lf_checker_rt::callee_thiscall!(10, u32, desc.as_mut_ptr() as u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, desc.as_mut_ptr() as u32);
            let _: u32 = lf_checker_rt::callee_cdecl!(12, u32, desc.as_mut_ptr() as u32);
            desc[0] = first_word;
            let shared = rd32(lf_checker_rt::relocated(SHARED_SLOT));
            // The original passes two pointers eight bytes apart (this at +8);
            // the this pointer is skipped as a frame address, arg0 is snapped.
            let desc_this = (desc.as_mut_ptr() as u32).wrapping_add(8);
            let assembled: u32 = lf_checker_rt::callee_thiscall!(
                13, u32, desc_this, desc.as_mut_ptr() as u32, arg0, arg1, arg2, arg4, arg5,
                handle
            );
            let opened: u32 = lf_checker_rt::callee_thiscall!(14, u32, assembled);
            let out: u32 = lf_checker_rt::callee_thiscall!(
                15,
                u32,
                arg6,
                region_ptr,
                reg_buf_a.as_mut_ptr() as u32,
                shared,
                reg_buf_b.as_mut_ptr() as u32,
                reg_buf_c.as_mut_ptr() as u32,
                0u32,
                0u32,
                reg_buf_d.as_mut_ptr() as u32,
                opened
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(16, u32,);
            out
        }
    }
);
