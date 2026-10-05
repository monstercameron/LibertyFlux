// original: 0x00a72a10 peds_tasks_gate_float_scaled_call (proposed)

/// Gate on three polled callees, then issue a float-scaled `thiscall/4`
/// request, with a constant fallback when a table slot is armed.
///
/// Zero-argument cdecl returning a byte in AL. A()==0, B()==0 or C()==0
/// returns AL=0 (upper EAX bits are the callee's leftovers on the A path,
/// zero otherwise). Otherwise two global mode bytes give
/// `(b1,b2)=(mode,0xFF)` when FLAG==0 else `(1,mode)`; each is stored as a
/// byte and re-read as a dword whose upper bytes were never written (zero
/// under the checker's stack fill). `bl` is set when D(scaled)!=0 and
/// `[D+0xC]==1`, where `scaled=[obj+12*([obj+0x2B0]+3)+0x2B0]`.
/// `f=G1*G2` is formed in the original's operand order, truncated toward
/// zero through the x87 with round-to-zero (NaN, +overflow and -overflow
/// all yield the indefinite's low dword 0, matching `as` everywhere
/// except `f >= 2^63`, which is pinned to 0), and its low dword doubled.
/// E1=`thiscall(h+0x26F8, scaled,0,l1,l2)` (the original pushes its four
/// words in forward order, so the parameter order is the reverse of the
/// push order); a nonzero answer returns 1. Else, when `bl` is clear,
/// return 0; otherwise E2=`thiscall(h+0x2C08, 0xC8,0,l1,l2)` decides.
///
/// Original: 0x00a72a10 (cdecl, no stack words; AL is the channel).
lf_checker_rt::export!(cdecl, rw_00a72a10() -> u32 {
    unsafe {
        const FLAG: u32 = 0x012FA6EA;
        const MODE: u32 = 0x0103CE48;
        const GFLOAT_A: u32 = 0x011735BC;
        const GFLOAT_B: u32 = 0x00FE8C58;
        const TWO63_F: f32 = 9223372036854775808.0; // 2^63, exactly representable
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let a = lf_checker_rt::callee_cdecl!(1, u32,);
        if (a as u8) == 0 {
            return a & 0xFFFF_FF00; // (an instruction of the original)
        }
        let obj = lf_checker_rt::callee_cdecl!(2, u32,);
        if obj == 0 {
            return 0;
        }
        let h = lf_checker_rt::callee_thiscall!(3, u32, obj);
        if h == 0 {
            return 0;
        }
        let flag: u8 = lf_checker_rt::global::<u8>(FLAG).read();
        let mode: u8 = lf_checker_rt::global::<u8>(MODE).read();
        // Byte locals re-read as dwords; the upper bytes were never written
        // (each byte sits alone in its dword slot) and are uninitialised
        // stack (zero under stack_fill 0 on both sides).
        let (b1, b2): (u32, u32) = if flag == 0 {
            (mode as u32, 0xFF)
        } else {
            (1, mode as u32)
        };
        let local1 = b1;
        let local2 = b2;
        let v = (obj.wrapping_add(0x2B0) as *const u32).read_unaligned();
        let slot = obj
            .wrapping_add(0x2B0)
            .wrapping_add(12u32.wrapping_mul(v.wrapping_add(3)));
        let darg = (slot as *const u32).read_unaligned();
        let d = lf_checker_rt::callee_cdecl!(4, u32, darg);
        let mut bl = 0u32;
        if d != 0 && (d.wrapping_add(0xC) as *const u32).read_unaligned() == 1 {
            bl = 1;
        }
        let g1 = f32::from_bits(lf_checker_rt::global::<u32>(GFLOAT_A).read());
        let g2 = f32::from_bits(lf_checker_rt::global::<u32>(GFLOAT_B).read());
        // NOTE (verified against the call log): the original pushes its
        // four E-call words in forward order, so the callee's parameter
        // order is the reverse: (scaled, 0, local1, local2).
        let f = mul(g1, g2);
        // x87 fistp-to-qword with round-toward-zero: NaN, +overflow and
        // -overflow all yield the indefinite 0x8000_0000_0000_0000 (low
        // dword 0); Rust `as` saturates instead, so only f >= 2^63 needs
        // the fix (every other input truncates identically, NaN included,
        // whose low dword is 0 on both).
        let low: u32 = if f >= TWO63_F { 0 } else { (f as i64) as u32 };
        let scaled = low.wrapping_mul(2);
        let e1 = lf_checker_rt::callee_thiscall!(5, u32, h.wrapping_add(0x26F8), scaled, 0, local1, local2);
        if (e1 as u8) != 0 {
            return 1;
        }
        if bl == 0 {
            return 0;
        }
        let e2 = lf_checker_rt::callee_thiscall!(5, u32, h.wrapping_add(0x2C08), 0xC8, 0, local1, local2);
        if (e2 as u8) != 0 { 1 } else { 0 }
    }
});
