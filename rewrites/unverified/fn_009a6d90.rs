// original: 0x009A6D90 audio_forward_3arg (proposed)

/// Builds an emit frame on the stack and forwards three arguments.
///
/// thiscall, three stack words (`arg0`, `arg1`, `arg2`). Constructs a
/// `FRAME_WORDS` (18-word) frame in place through the frame constructor
/// (callee 1, thiscall on the frame address), stores `arg2` at frame word
/// `ARG2_SLOT` (+0x14), then calls the emitter (callee 2, thiscall on
/// `this`) with (`arg0`, `arg1`, frame address). Returns the emitter's
/// answer. The frame address differs between the two sides, so the
/// contract skips that call argument and snapshots frame words 2-17
/// instead; the constructor's writes are scripted so both sides start
/// from identical frame bytes.
lf_checker_rt::export!(thiscall, rw_009a6d90(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const CTOR: u32 = 1;
        const EMIT: u32 = 2;
        const FRAME_WORDS: usize = 18;
        const ARG2_SLOT: usize = 5;
        let mut frame = [0u32; FRAME_WORDS];
        let ptr = frame.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, ptr);
        frame[ARG2_SLOT] = arg2;
        lf_checker_rt::callee_thiscall!(EMIT, u32, this, arg0, arg1, ptr)
    }
});
