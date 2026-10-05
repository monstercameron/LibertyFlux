// original: 0x00AD1270 audio_init_gated (proposed)

/// Bring up the audio parameter path when the audio gate is open.
///
/// Reads the gate flag at `GATE`; returns immediately unless it equals 1.
/// Otherwise it copies two 16-byte coefficient blocks from read-only tables
/// into scratch, applies them through the parameter applier (id 1, selector
/// 0) and the parameter commit (id 3), fetches two probe answers (id 2),
/// forwards the second through the voice hook (id 4, thiscall on the object
/// at `CFG_THIS` with the word at `CFG_WORD` alongside), emits one mode
/// command (id 5) and twelve constant two-word config commands (id 6), then
/// takes the thread-session reference (slot index from `TLS_SLOT_IDX`,
/// count at +0x0c of the thread object): on the first reference it opens
/// the session (id 7), submits a ten-word descriptor whose pointer argument
/// aims at scratch holding `SENTINEL` followed by the second coefficient
/// block (id 8), and on release to zero it closes the session (id 9).
/// Finishes with three no-argument finalisers (ids 10-12).
///
/// Original: 0x00AD1270 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ad1270() -> u32 {
    unsafe {
        const GATE: u32 = 0x0103_F490;
        const TLS_SLOT_IDX: u32 = 0x017A_BA14;
        const CFG_THIS: u32 = 0x0154_E190;
        const CFG_WORD: u32 = 0x0154_E24C;
        const COEFFS_A: u32 = 0x00EA_69A0;
        const COEFFS_B: u32 = 0x00FE_8F20;
        const THREAD_REFCOUNT: u32 = 0x0c;
        const SENTINEL: u32 = 0xffff_ffff;
        const APPLY: u32 = 1;
        const PROBE: u32 = 2;
        const COMMIT: u32 = 3;
        const VOICE_HOOK: u32 = 4;
        const MODE: u32 = 5;
        const CONFIG: u32 = 6;
        const SESSION_OPEN: u32 = 7;
        const SUBMIT: u32 = 8;
        const SESSION_CLOSE: u32 = 9;
        const FIN1: u32 = 10;
        const FIN2: u32 = 11;
        const FIN3: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if lf_checker_rt::global::<u32>(GATE).read_unaligned() != 1 {
            return 0;
        }
        let mut area = [0u32; 8];
        let base_a = lf_checker_rt::relocated(COEFFS_A);
        let base_b = lf_checker_rt::relocated(COEFFS_B);
        for i in 0..4 {
            area[3 + i] = rd32(base_a + (i as u32) * 4);
        }
        let coeff_ptr = (&area[3] as *const u32) as u32;
        lf_checker_rt::callee_cdecl!(APPLY, u32, 0, coeff_ptr);
        for i in 0..4 {
            area[3 + i] = rd32(base_b + (i as u32) * 4);
        }
        let first = lf_checker_rt::callee_cdecl!(PROBE, u32,);
        lf_checker_rt::callee_cdecl!(COMMIT, u32, 4, coeff_ptr, first);
        let second = lf_checker_rt::callee_cdecl!(PROBE, u32,);
        let this = lf_checker_rt::global::<u32>(CFG_THIS).read_unaligned();
        let word = lf_checker_rt::global::<u32>(CFG_WORD).read_unaligned();
        lf_checker_rt::callee_thiscall!(VOICE_HOOK, u32, this, word, second);
        lf_checker_rt::callee_cdecl!(MODE, u32, 0x15, 3);
        const CONFIGS: [(u32, u32); 12] = [
            (0x0f, 0),
            (0x0a, 1),
            (0x06, 1),
            (0x05, 1),
            (0x13, 1),
            (0x1a, 0xff),
            (0x19, 0xff),
            (0x18, 7),
            (0x17, 1),
            (0x16, 2),
            (0x14, 0),
            (0x15, 0),
        ];
        for i in 0..CONFIGS.len() {
            lf_checker_rt::callee_cdecl!(CONFIG, u32, CONFIGS[i].0, CONFIGS[i].1);
        }
        let slot = lf_checker_rt::global::<u32>(TLS_SLOT_IDX).read_unaligned();
        let obj = lf_checker_rt::tls_slot(slot as usize);
        let refc = (obj + THREAD_REFCOUNT) as *mut u32;
        refc.write_unaligned(refc.read_unaligned().wrapping_add(1));
        if refc.read_unaligned() == 1 {
            lf_checker_rt::callee_cdecl!(SESSION_OPEN, u32, 1);
        }
        area[2] = SENTINEL;
        let submit_ptr = (&area[2] as *const u32) as u32;
        const ONE: u32 = 0x3f80_0000;
        lf_checker_rt::callee_cdecl!(SUBMIT, u32, 0, 0, ONE, ONE, 0, 0, 0, ONE, ONE, submit_ptr);
        let slot2 = lf_checker_rt::global::<u32>(TLS_SLOT_IDX).read_unaligned();
        let obj2 = lf_checker_rt::tls_slot(slot2 as usize);
        let refc2 = (obj2 + THREAD_REFCOUNT) as *mut u32;
        refc2.write_unaligned(refc2.read_unaligned().wrapping_sub(1));
        if refc2.read_unaligned() == 0 {
            lf_checker_rt::callee_cdecl!(SESSION_CLOSE, u32,);
        }
        lf_checker_rt::callee_cdecl!(FIN1, u32,);
        lf_checker_rt::callee_cdecl!(FIN2, u32,);
        lf_checker_rt::callee_cdecl!(FIN3, u32,);
        0
    }
});
