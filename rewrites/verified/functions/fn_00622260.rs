// original: 0x00622260 net_dispatch_result (proposed)

/// Dispatch a session result: query the result code, then either transform or
/// forward the token packet built from it.
///
/// `this` is the session object; `a` and `c` are opaque tokens, `b` a value
/// carried in the packet, `d` an optional native-call token. The packet lays
/// out as (token 0, token 1, 0, b, 0, d). The object holds
/// a context word at `+CTX`, a handle at `+HDL` and two token words at
/// `+TOK0/TOK1`. Callee 1 is asked first with (token a, handle); its answer
/// selects the path. A five-word packet (token 0, token 1, b, 0, d) is built,
/// and when `d` is non-zero a three-word native call notifies (scratch, c, d).
///
/// A negative answer runs the transform path: callee 3 is asked with the
/// packet, a scratch buffer, 0x100 and an out slot, and writes the slot; a
/// zero low byte returns its full answer, otherwise callee 4 is asked with
/// (token a, handle, scratch, slot word) and its answer is returned. A
/// non-negative answer runs the forward path: callee 5 is asked with
/// (answer, packet, leftover 0, leftover 1) and its answer is returned; the
/// callee pops all four words, which balances the eight bytes below the
/// pushes, and those two words still hold the previous call's pushed
/// arguments ((c, d) when the native call ran, else (token a, handle)),
/// passed on identically by the rewrite.
///
/// The rewrite tail-calls the intercepted security-cookie check before
/// returning, matching the original's epilogue. Untouched scratch pointers
/// are skipped from the call comparison; the built packet words are
/// snapshotted through both callees that receive the packet.
///
/// Original: 0x00622260 (thiscall, four stack words), returns the answering
/// callee's value.
lf_checker_rt::export!(thiscall, rw_00622260(this: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x24;
        const HDL: u32 = 0x32f4;
        const TOK0: u32 = 0x540;
        const TOK1: u32 = 0x544;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        unsafe fn body(this: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
            unsafe {
                let r: u32 =
                    lf_checker_rt::callee_thiscall!(1, u32, rd32(this + CTX), a, rd32(this + HDL));
                let mut pkt = [rd32(this + TOK0), rd32(this + TOK1), 0, b, 0, d];
                if d != 0 {
                    let mut nbuf = [0u32; 1];
                    lf_checker_rt::callee_cdecl!(2, u32, nbuf.as_mut_ptr() as u32, c, d);
                }
                if (r as i32) >= 0 {
                    // The last two words are the stack slots below the pushes,
                    // still holding the previous call's pushed arguments: the
                    // native call's (c, d) when it ran, else the query's
                    // (token a, handle). Same inputs, same values, both sides.
                    let (s0, s1) = if d != 0 {
                        (c, d)
                    } else {
                        (a, rd32(this + HDL))
                    };
                    return lf_checker_rt::callee_thiscall!(5, u32, rd32(this + CTX), r,
                        pkt.as_mut_ptr() as u32, s0, s1);
                }
                let mut wslot = 0u32;
                let mut sbuf = [0u32; 1];
                let t: u32 = lf_checker_rt::callee_thiscall!(3, u32, pkt.as_mut_ptr() as u32,
                    sbuf.as_mut_ptr() as u32, 0x100, &mut wslot as *mut u32 as u32);
                if (t as u8) == 0 {
                    return t;
                }
                lf_checker_rt::callee_thiscall!(4, u32, rd32(this + CTX), a,
                    rd32(this + HDL), sbuf.as_mut_ptr() as u32, wslot)
            }
        }

        let ans = body(this, a, b, c, d);
        lf_checker_rt::callee_cdecl!(6, u32,);
        ans
    }
});
