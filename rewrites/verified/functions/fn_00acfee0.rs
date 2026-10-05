// original: 0x00ACFEE0 audio_voice_start_param (proposed)

/// Start a voice with a normalised position parameter.
///
/// When the reference level is ordered-above the position and no tag byte
/// is given, returns at once (the caller's leftover eax, pinned to 0 in
/// the proof). Otherwise normalises the position as
/// `(pos - ref) / (span - ref)` in that operation order, picks mode 3
/// when the voice tune ordered-equals its reference and mode 2 otherwise,
/// resolves the tag (the tag byte when set, else the registry's virtual
/// gate, slot 5, applied to the voice key and truncated to a half-word),
/// looks the voice up (thiscall/2: owner, mode, tag) and starts it
/// (thiscall/5: voice, s0, lookup answer, normalised position bits, 1,
/// s3). Thiscall/4; the original overwrites its incoming position slot
/// with the quotient (dead, callee-popped), so the proof leaves the stack
/// check off. Returns the start answer.
lf_checker_rt::export!(thiscall, rw_00acfee0(this: u32, s0: u32, s1: u32, s2: u32, s3: u32) -> u32 {
    unsafe {
        const VT_GATE: u32 = 1;
        const LOOKUP: u32 = 2;
        const START: u32 = 3;
        const REF: u32 = 0x00ECA7F4;
        const SPAN: u32 = 0x00FE88E8;
        const TUNE_REF: u32 = 0x00FE8628;
        const REGISTRY: u32 = 0x018B8968;
        const OWNER: u32 = 0x016D9F58;
        const KEY_OFF: u32 = 0xD8;
        const TUNE_OFF: u32 = 0x160;
        const GATE_SLOT: u32 = 0x14;
        const TAG_OFF: u32 = 0x20;
        const DIRECT_TAG: u32 = 0x22;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let r = lf_checker_rt::global::<f32>(REF).read();
        let v = f32::from_bits(s1);
        if r > v && (s2 as u8) == 0 {
            return 0;
        }
        let span = lf_checker_rt::global::<f32>(SPAN).read();
        let q = div(sub(v, r), sub(span, r));
        let tune = (this.wrapping_add(TUNE_OFF) as *const f32).read();
        let tune_ref = lf_checker_rt::global::<f32>(TUNE_REF).read();
        let mode = if tune == tune_ref { 3u32 } else { 2u32 };
        let tag = if (s2 as u8) != 0 {
            DIRECT_TAG
        } else {
            let reg = lf_checker_rt::global::<u32>(REGISTRY).read();
            let key = (this.wrapping_add(KEY_OFF) as *const u32).read();
            let vt = (reg as *const u32).read();
            let target = (vt.wrapping_add(GATE_SLOT) as *const u32).read();
            let gate: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(target as usize);
            let ans = gate(key);
            (ans.wrapping_add(TAG_OFF) as *const u16).read() as u32
        };
        let found =
            lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(OWNER), mode, tag);
        lf_checker_rt::callee_thiscall!(START, u32, this, s0, found, q.to_bits(), 1u32, s3)
    }
});
