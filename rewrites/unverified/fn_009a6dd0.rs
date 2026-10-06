// original: 0x009A6DD0 audio_emit_indexed (proposed)

/// Emits through an indexed voice table, selecting the voice by id bytes.
///
/// thiscall, two stack words (`arg0`, `arg1`). Builds a frame and calls
/// the emitter (callee 1, thiscall on `this`) with (`arg0`, `arg1`, frame
/// address). When the emitter answers null, returns 0. Otherwise reads id
/// bytes from the answer: `kind` at `ID_KIND` (+4) and `bank` at `ID_BANK`
/// (+0x40). A `kind` of `NO_VOICE` (0xFF, an equality compare) selects the
/// null voice; otherwise the voice address is `G1 * kind + table`, where
/// `G1` is the stride global at `STRIDE`, and `table` is the dword at
/// `TABLE_BASE + bank * BANK_STRIDE + TABLE_OFF` with the base global at
/// `TABLE` (`TABLE_OFF` 0x6F14, `BANK_STRIDE` 0x6F40; all arithmetic wraps
/// mod 2^32). The chosen voice (or null) is then activated through
/// callee 2 (thiscall on the voice, one zero word). Returns the
/// activation's answer. The frame address differs between the two sides,
/// so the contract skips that call argument and snapshots frame words 2-17.
lf_checker_rt::export!(thiscall, rw_009a6dd0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const CTOR: u32 = 1;
        const EMIT: u32 = 2;
        const ACTIVATE: u32 = 3;
        const FRAME_WORDS: usize = 18;
        const ID_KIND: u32 = 4;
        const ID_BANK: u32 = 0x40;
        const NO_VOICE: u8 = 0xFF;
        const STRIDE: u32 = 0x00115D968;
        const TABLE: u32 = 0x00115D988;
        const TABLE_OFF: u32 = 0x6F14;
        const BANK_STRIDE: u32 = 0x6F40;
        let mut frame = [0u32; FRAME_WORDS];
        let ptr = frame.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, ptr);
        let answer: u32 = lf_checker_rt::callee_thiscall!(EMIT, u32, this, arg0, arg1, ptr);
        if answer == 0 {
            return 0;
        }
        let kind = (answer.wrapping_add(ID_KIND) as *const u8).read();
        let voice = if kind == NO_VOICE {
            0
        } else {
            let bank = (answer.wrapping_add(ID_BANK) as *const u8).read() as u32;
            let stride = lf_checker_rt::global::<u32>(STRIDE).read_unaligned();
            let base = lf_checker_rt::global::<u32>(TABLE).read_unaligned();
            let slot = base.wrapping_add(bank.wrapping_mul(BANK_STRIDE)).wrapping_add(TABLE_OFF);
            stride.wrapping_mul(kind as u32).wrapping_add((slot as *const u32).read_unaligned())
        };
        lf_checker_rt::callee_thiscall!(ACTIVATE, u32, voice, 0)
    }
});
