// original: 0x00d8bb70 audio_voice_param_init (proposed)

/// Program one audio voice (or effect) with its default parameter block.
///
/// Takes no arguments (cdecl, no stack words). It issues seven calls: two to
/// a two-pointer helper over one five-word block (second call first, words
/// 1-3 reworked with a global float between the calls), then three
/// constant setup calls,
/// then a query whose x87 float answer `q` is scaled and biased,
/// `x = q * MUL + ADD`, and handed to a final five-word programming call
/// together with two all-ones flags and a second image address. The two
/// address operands are relocated image pointers, not integer handles. The
/// float operation order is the original's. Returns the final call's answer.
///
/// Original: 0x00d8bb70 (cdecl, no arguments; returns last call's result).
lf_checker_rt::export!(cdecl, rw_00d8bb70() -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        const SETUP2: u32 = 2;
        const SETUP1: u32 = 3;
        const PROGRAM: u32 = 4;
        const QUERY: u32 = 5;
        const HANDLE_A_FILE: u32 = 0x01797764;
        const HANDLE_B_FILE: u32 = 0x0179bf90;
        const MUL_ADDR: u32 = 0x00fe8908;
        const ADD_ADDR: u32 = 0x00fe87cc;

        const BLK0: u32 = 0x8000_0000;
        const BLK1: u32 = 0x3dcc_cccd;
        const BLK2: u32 = 0x3f83_d70a;
        const BLK3: u32 = 0x3f26_6667;
        const BLK4: u32 = 0x3f73_3333;
        const BLK0_B: u32 = 0xc4ff_ffff;
        const K_ADDR: u32 = 0x00fe876c;
        let mut buf = [BLK0, BLK1, BLK2, BLK3, BLK4];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            HELPER, u32, buf.as_mut_ptr().wrapping_add(1) as u32, buf.as_mut_ptr() as u32
        );
        // Between the calls the block is reworked with one global float:
        // word 0 becomes a constant, word 1 adds it, words 2 and 3
        // subtract it (second operand order pinned for NaN identity).
        let k = f32::from_bits((lf_checker_rt::relocated(K_ADDR) as *const u32).read_unaligned());
        buf[0] = BLK0_B;
        buf[1] = (core::hint::black_box(f32::from_bits(buf[1])) + core::hint::black_box(k)).to_bits();
        buf[2] = (core::hint::black_box(f32::from_bits(buf[2])) - core::hint::black_box(k)).to_bits();
        buf[3] = (core::hint::black_box(f32::from_bits(buf[3])) - core::hint::black_box(k)).to_bits();
        let _: u32 = lf_checker_rt::callee_cdecl!(
            HELPER, u32, buf.as_mut_ptr().wrapping_add(1) as u32, buf.as_mut_ptr() as u32
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(SETUP2, u32, 0x3ecccccd, 0x3f19999a);
        let _: u32 = lf_checker_rt::callee_cdecl!(SETUP1, u32, 0xff000000);
        // The two "handles" are image addresses: the push sites carry
        // relocations, so the original passes the relocated values.
        let handle_a = lf_checker_rt::relocated(HANDLE_A_FILE);
        let handle_b = lf_checker_rt::relocated(HANDLE_B_FILE);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            PROGRAM, u32, 0x3e428f5c, 0x3f828f5c, handle_a, 0xffffffff, 0xffffffff
        );
        let q: f32 = lf_checker_rt::callee_cdecl!(QUERY, f32, handle_a, 1);
        let mul = f32::from_bits((lf_checker_rt::relocated(MUL_ADDR) as *const u32).read_unaligned());
        let add = f32::from_bits((lf_checker_rt::relocated(ADD_ADDR) as *const u32).read_unaligned());
        let x = core::hint::black_box(
            core::hint::black_box(q) * core::hint::black_box(mul),
        ) + core::hint::black_box(add);
        lf_checker_rt::callee_cdecl!(PROGRAM, u32, x.to_bits(), 0x3f828f5c, handle_b, 0xffffffff, 0xffffffff)
    }
});
