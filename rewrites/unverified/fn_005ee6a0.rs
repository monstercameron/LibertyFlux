// original: 0x005ee6a0 text_run_resolve_and_sink (proposed)

/// Resolve a text run's style, scale its box, and sink it.
///
/// `this` is the format context (a pointer at `+0x10`, bias and divisors at
/// `+0x14`/`+0x18`/`+0x1c`); `arg0` is the run record. A style word is
/// selected (`[arg0+0xe0]`, or `0xfc9c85` when the word at `+0xe4` is zero)
/// and passed with `0x3a` to callee 1 (stdcall); a non-zero answer plus one,
/// else the same selection, gives the style id. Callee 3 (thiscall on the
/// run, a frame byte-slot and the context) then classifies the run: a zero
/// byte takes a short path (callees 1 and 2 again, the latter fed through a
/// `[ctx+0x10]` chain read), a non-zero byte the long path.
///
/// The long path resolves the style id through two table callees, then tests
/// bit `0x80` of a byte picked by that answer from a global table: when set,
/// the following pointer is null and the function faults reading through it
/// (kept as fault parity); otherwise the pointer is scaled by the answer.
/// Callee 1 (stdcall) and callee 2 (stdcall) run, and thread-local slot 0's
/// dword at `+0x8cc`, when non-zero, runs an allocator/registrar trio
/// (callees 6-8, the middle one thiscall on callee 6's answer).
///
/// When callee 2's answer is zero the function returns it. Otherwise four box
/// floats are combined, a global function pointer is polled four times with
/// sentinel picks exactly like the sibling measurer, and one product of the
/// third edge with the picked integer, scaled by `1/[ctx+0x18]`, is kept
/// (three sibling products and two constants are computed and dropped).
/// The same thread-local flag picks the sink: non-zero calls callee 10
/// (EDX plus three frame words, caller cleanup); zero zeroes two frame
/// slots and calls callees 11-14 (thiscall, thiscall, cdecl-3, thiscall, the
/// frame addresses uncompared). A stack-cookie check call ends every path
/// and preserves the return value.
///
/// Original: 0x005ee6a0 (thiscall, ECX = this, one stack word; callees 1-2
/// pop their stack words, callee 10 is EDX plus stack with caller cleanup).
lf_checker_rt::export!(thiscall, rw_005ee6a0(this: u32, arg0: u32) -> u32 {
    unsafe {
        // Default style pointer: carries a relocation entry, so the running
        // original sees the relocated value.
        const STYLE_ALT: u32 = 0x00fc9c85;
        const STYLE_OFF: u32 = 0xe0;
        const STYLE_GATE: u32 = 0xe4;
        const TLS_FLAG: u32 = 0x8cc;
        const G_CALLEE_PTR: u32 = 0x00e731ac;
        const G_SENTINEL: u32 = 0x0110dd14;
        const G_SELP0: u32 = 0x0105c880;
        const G_SELP1: u32 = 0x0105c87c;
        const G_SELQ0: u32 = 0x0105c884;
        const G_SELQ1: u32 = 0x0105c888;
        const G_DX: u32 = 0x011764c0;
        const G_ONE: u32 = 0x00fe88e8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let sel: u32 = if rd16(arg0.wrapping_add(STYLE_GATE)) == 0 {
            lf_checker_rt::relocated(STYLE_ALT)
        } else {
            rd32(arg0.wrapping_add(STYLE_OFF))
        };
        let r1: u32 = lf_checker_rt::callee_cdecl!(16, u32, sel, 0x3a);
        let mut ebx = if r1 != 0 { r1.wrapping_add(1) } else { sel };

        let mut flagslot = 0u32;
        lf_checker_rt::callee_thiscall!(
            3, u32, arg0,
            &mut flagslot as *mut u32 as u32,
            this
        );
        // EAX at the ebx==0 exit: callee 2's answer, unless the registrar
        // trio ran and left callee 8's answer instead.
        let mut tail_eax = 0u32;
        if (flagslot & 0xff) == 0 {
            let rd: u32 = lf_checker_rt::callee_stdcall!(1, u32, ebx, 0);
            // Faithful chain read (stdcall ignores ECX; always mapped here).
            let _chain = rd32(rd32(this.wrapping_add(0x10)).wrapping_add(0x0c));
            let _ = _chain;
            ebx = lf_checker_rt::callee_stdcall!(2, u32, rd);
            tail_eax = ebx;
        } else {
            let rb: u32 = lf_checker_rt::callee_cdecl!(
                4, u32,
                &flagslot as *const u32 as u32
            );
            let rc: u32 = lf_checker_rt::callee_cdecl!(5, u32, rb);
            let dx = g32(G_DX);
            let bittab = rd32(dx.wrapping_add(4));
            let esi: u32 = if rd8(bittab.wrapping_add(rc)) & 0x80 == 0 {
                rd32(dx.wrapping_add(12)).wrapping_mul(rc).wrapping_add(rd32(dx))
            } else {
                0
            };
            let rd: u32 = lf_checker_rt::callee_stdcall!(1, u32, ebx, 0);
            // Fault parity with the original's null read on the set-bit path.
            let _probe = rd32(esi);
            let _ = _probe;
            ebx = lf_checker_rt::callee_stdcall!(2, u32, rd);
            tail_eax = ebx;
            let tls0 = lf_checker_rt::tls_slot(0);
            if rd32(tls0.wrapping_add(TLS_FLAG)) != 0 {
                let rf: u32 = lf_checker_rt::callee_cdecl!(6, u32, 8, 0);
                let rg: u32 = if rf != 0 {
                    lf_checker_rt::callee_thiscall!(7, u32, rf, rc)
                } else {
                    0
                };
                tail_eax = lf_checker_rt::callee_cdecl!(8, u32, rg);
            }
        }

        let retv: u32 = if ebx == 0 {
            tail_eax
        } else {
            let f30 = rdf(arg0.wrapping_add(0x30));
            let bias = rdf(this.wrapping_add(0x14));
            let s20 = sub(f30, bias);
            let s2c = rdf(arg0.wrapping_add(0x2c));
            let s28 = add(rdf(arg0.wrapping_add(0x20)), s2c);
            let s24 = sub(add(rdf(arg0.wrapping_add(0x24)), f30), bias);
            let fp = g32(G_CALLEE_PTR);
            let poll: extern "cdecl" fn() -> u32 = core::mem::transmute(fp as usize);
            let sent = g32(G_SENTINEL);
            let ebp_sel = if sent == poll() { g32(G_SELP1) } else { g32(G_SELP0) };
            let ebx_sel = if sent == poll() { g32(G_SELQ1) } else { g32(G_SELQ0) };
            let edi_sel = if sent == poll() { g32(G_SELP1) } else { g32(G_SELP0) };
            let ecx_sel = if sent == poll() { g32(G_SELQ1) } else { g32(G_SELQ0) };
            let one: f32 = f32::from_bits(g32(G_ONE));
            let x2 = div(one, rdf(this.wrapping_add(0x18)));
            let x3 = div(one, rdf(this.wrapping_add(0x1c)));
            let s40 = mul(ecx_sel as i32 as f32, mul(x2, s2c));
            // Sibling products, computed and dropped like the original.
            let _s48 = mul(mul(x2, s28), ebx_sel as i32 as f32);
            let _s4c = mul(mul(x3, s20), edi_sel as i32 as f32);
            let _s44 = mul(mul(x3, s24), ebp_sel as i32 as f32);
            let tls0 = lf_checker_rt::tls_slot(0);
            if rd32(tls0.wrapping_add(TLS_FLAG)) != 0 {
                let mut s1c = 0xffffffffu32;
                let mut s30 = 0u32;
                let mut s40v = s40;
                let mut s18 = ebx;
                lf_checker_rt::callee_fastcall!(
                    10, u32, ecx_sel,
                    &mut s18 as *mut u32 as u32,
                    &mut s1c as *mut u32 as u32,
                    &mut s30 as *mut u32 as u32,
                    &mut s40v as *mut f32 as u32
                )
            } else {
                let mut s14z = 0u32;
                lf_checker_rt::callee_thiscall!(
                    11, u32,
                    &mut s14z as *mut u32 as u32,
                    ebx
                );
                let s10z = 0u32;
                lf_checker_rt::callee_thiscall!(
                    12, u32,
                    &s10z as *const u32 as u32,
                );
                let mut s1c = 0xffffffffu32;
                let mut s2cv = s2c;
                let mut s3c = 0u32;
                lf_checker_rt::callee_cdecl!(
                    13, u32,
                    &mut s1c as *mut u32 as u32,
                    &mut s2cv as *mut f32 as u32,
                    &mut s3c as *mut u32 as u32
                );
                let mut s14z2 = 0u32;
                lf_checker_rt::callee_thiscall!(
                    14, u32,
                    &mut s14z2 as *mut u32 as u32,
                )
            }
        };
        // The stack-cookie check: preserves everything, logged for the sequence.
        let _ck: u32 = lf_checker_rt::callee_cdecl!(15, u32,);
        retv
    }
});
