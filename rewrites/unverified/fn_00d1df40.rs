// original: 0x00d1df40 cover_stance_classifier (proposed)
/// Decide a cover-stance code from a ped object, a float input and two flag
/// bytes, through a 4-key table lookup.
///
/// `obj` is the ped. Callee 1 (thiscall on `obj+0x2b0`, one stack arg) is
/// asked first: a non-zero answer means stance bit `bl` is set. Otherwise
/// callee 2 (thiscall on the same member, no stack args) is fetched twice;
/// a null first fetch clears `bl`, else callee 3 (cdecl) maps the word at
/// `+0x18` of the fetch and bit 13 of the word at `+0x20` of its answer
/// becomes `bl`.
///
/// The virtual slot at `+0x128` of `obj` (thiscall, no stack args, planted
/// stub) then picks a threshold: 4.0 when it answers non-zero, 2.5 when
/// zero. `level` is compared against it (`comiss` order: set when the
/// threshold is below the level or either is NaN) to form bit `bx`.
/// `flag_a`/`flag_b` are normalised to 0/1 from their low bytes (the
/// original writes the normalised values back over its own argument slots;
/// only the values are behaviour). When the mode global holds 2, `bl` is
/// set, `bx` is clear and `level` exceeds 3.5, the bits flip to `bl`=0,
/// `bx`=1.
///
/// The four bits index the row table: rows of five words (four key words
/// then the stance code), terminated by a code of -1. A row matching
/// (`flag_b`, `flag_a`, `bl`, `bx`) yields its code; row 0 holding -1, or no
/// match before the terminator, yields -1. Returns the stance code.
/// Original: 0x00d1df40 (cdecl, four stack args).
lf_checker_rt::export!(cdecl, rw_00d1df40(obj: u32, level: u32, flag_a: u32, flag_b: u32) -> u32 {
    unsafe {
        const MEMBER: u32 = 0x2b0;
        const VT_SLOT: u32 = 0x128;
        const THRESH_HI: u32 = 0x00fe8ab8;
        const THRESH_LO: u32 = 0x00fe8a60;
        const OVERRIDE_LVL: u32 = 0x00fe8ab0;
        const MODE: u32 = 0x011d6fd4;
        const TABLE: u32 = 0x01054448;
        const ASK: u32 = 1;
        const FETCH: u32 = 2;
        const MAP: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdglobal(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read() }
        }

        let bl = {
            let ok: u32 = lf_checker_rt::callee_thiscall!(ASK, u32, obj + MEMBER, 1);
            if (ok & 0xff) != 0 {
                1
            } else {
                let first: u32 = lf_checker_rt::callee_thiscall!(FETCH, u32, obj + MEMBER);
                if first == 0 {
                    0
                } else {
                    let second: u32 = lf_checker_rt::callee_thiscall!(FETCH, u32, obj + MEMBER);
                    let mapped: u32 = lf_checker_rt::callee_cdecl!(MAP, u32, rd32(second + 0x18));
                    (rd32(mapped + 0x20) >> 13) & 1
                }
            }
        };
        let vtable = rd32(obj);
        let hook: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable + VT_SLOT) as usize);
        let picked = hook(obj);
        let threshold = f32::from_bits(rdglobal(if (picked & 0xff) != 0 { THRESH_HI } else { THRESH_LO }));
        let level_f = f32::from_bits(level);
        let norm_b = ((flag_b & 0xff) != 0) as u32;
        let norm_a = ((flag_a & 0xff) == 0) as u32;
        let mut edx = bl;
        let mut ebx = (!(threshold >= level_f)) as u32;
        if rdglobal(MODE) == 2 && edx == 1 && ebx == 0 && level_f > f32::from_bits(rdglobal(OVERRIDE_LVL)) {
            edx = 0;
            ebx = 1;
        }
        if rdglobal(TABLE + 0x10) as i32 == -1 {
            return 0xffff_ffff;
        }
        let mut ecx = 0u32;
        let mut eax = 0u32;
        loop {
            if rdglobal(TABLE + eax) == norm_b
                && rdglobal(TABLE + eax + 4) == norm_a
                && rdglobal(TABLE + eax + 8) == edx
                && rdglobal(TABLE + eax + 12) == ebx
            {
                return rdglobal(TABLE + eax + 16);
            }
            ecx += 1;
            eax = ecx * 20;
            if rdglobal(TABLE + eax + 16) as i32 == -1 {
                return 0xffff_ffff;
            }
        }
    }
});
