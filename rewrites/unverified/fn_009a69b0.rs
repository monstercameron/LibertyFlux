// original: 0x009A69B0 audio_emit_counted (proposed)

/// Emits a counted request, advancing the shared sequence counter.
///
/// thiscall, two stack words (`arg0`, `arg1`). Builds a `FRAME_WORDS`
/// (18-word) frame through the frame constructor (callee 1), stores `arg1`
/// at frame word `ARG1_SLOT` (+0x14) and the counter's old value at
/// `SEQ_SLOT` (+0x24), then advances the sequence global at `SEQ`
/// (`(old + STEP) mod PERIOD`, SIGNED 32-bit division, divisor 100, so no
/// trial can divide by zero or overflow) and sets frame flag bit
/// `READY_BIT` (bit 1 of the byte at +0x46). Finally calls the emitter
/// (callee 2, thiscall on `this`) with (`arg0`, `KIND`, frame address),
/// where `KIND` is the constant tag 0xE918BC. Returns the emitter's
/// answer. The frame address differs between the two sides, so the
/// contract skips that call argument and snapshots frame words 2-17 instead.
lf_checker_rt::export!(thiscall, rw_009a69b0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const CTOR: u32 = 1;
        const EMIT: u32 = 2;
        const FRAME_WORDS: usize = 18;
        const ARG1_SLOT: usize = 5;
        const SEQ_SLOT: usize = 9;
        const READY_OFF: usize = 0x46;
        const READY_BIT: u8 = 2;
        const SEQ: u32 = 0x0016CFD48;
        const STEP: i32 = 0x46;
        const PERIOD: i32 = 100;
        const KIND: u32 = 0x00E918BC;
        let mut frame = [0u32; FRAME_WORDS];
        let ptr = frame.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, ptr);
        let old = lf_checker_rt::global::<u32>(SEQ).read_unaligned();
        frame[ARG1_SLOT] = arg1;
        frame[SEQ_SLOT] = old;
        let next = (old as i32).wrapping_add(STEP) % PERIOD;
        let flag = (ptr as *mut u8).add(READY_OFF);
        flag.write(flag.read() | READY_BIT);
        lf_checker_rt::global::<u32>(SEQ).write_unaligned(next as u32);
        lf_checker_rt::callee_thiscall!(EMIT, u32, this, arg0, lf_checker_rt::relocated(KIND), ptr)
    }
});
