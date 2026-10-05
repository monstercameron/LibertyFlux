// original: 0x0088BF80 rage::audVoiceDSound::vf10

/// Current playback position of this DirectSound voice, or -1 when the
/// voice is not playing.
///
/// `this` points to the voice. Its own state gate (slot `+0x18` of its
/// table) runs first; when it reports false the answer is -1. Otherwise the
/// device at `+0x90` is asked for its play cursor (slot `+0x10`, called
/// with the device, an out word and zero), and the answer is the cursor
/// plus the frequency word at `+0xcc` scaled from 16.16 fixed point,
/// halved once more and biased by the word at `+0xc8`, converted through
/// the position helper (callee 3) together with the base at `+0xc`.
///
/// Original: 0x0088BF80 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088BF80(this: u32) -> u32 {
    unsafe {
        const OWN_GATE: u32 = 0x18;
        const VOICE_DEVICE: u32 = 0x90;
        const SLOT_CURSOR: u32 = 0x10;
        const FREQ: u32 = 0xcc;
        const BASE: u32 = 0x0c;
        const BIAS: u32 = 0xc8;
        const POS_HELPER: u32 = 3;

        let table = (this as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((table + OWN_GATE) as *const u32).read_unaligned() as usize
        );
        if gate(this) as u8 == 0 {
            return 0xffff_ffff;
        }
        let obj = ((this + VOICE_DEVICE) as *const u32).read_unaligned();
        let dtable = (obj as *const u32).read_unaligned();
        let cursor: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(
                ((dtable + SLOT_CURSOR) as *const u32).read_unaligned() as usize
            );
        let mut pos: u32 = 0;
        cursor(obj, &mut pos as *mut u32 as u32, 0);
        let freq = ((this + FREQ) as *const u32).read_unaligned();
        let base = ((this + BASE) as *const u32).read_unaligned();
        let bias = ((this + BIAS) as *const u32).read_unaligned();
        let scaled = (((freq >> 1) << 17).wrapping_add(pos) >> 1).wrapping_add(bias);
        lf_checker_rt::callee_cdecl!(POS_HELPER, u32, scaled, base)
    }
});
