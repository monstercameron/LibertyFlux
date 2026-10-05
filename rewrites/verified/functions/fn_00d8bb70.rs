// original: 0x00d8bb70 audio_voice_param_init (proposed)

/// Program one audio voice (or effect) with its default parameter block.
///
/// Takes no arguments (cdecl, no stack words). It issues seven calls: two to
/// a two-pointer helper (whose answers never reach an observed call, so the
/// frame shuffling between them is dead), then three constant setup calls,
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

        let mut p0 = 0x3dccccdcu32;
        let mut p1 = 0x80000000u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            HELPER, u32, &mut p0 as *mut u32 as u32, &mut p1 as *mut u32 as u32
        );
        // Dead frame shuffling between the helper calls is omitted: neither
        // its result nor the helper's answers reach any observed call.
        let _: u32 = lf_checker_rt::callee_cdecl!(
            HELPER, u32, &mut p0 as *mut u32 as u32, &mut p1 as *mut u32 as u32
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
