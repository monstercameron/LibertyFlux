// original: 0x00e5f9d0 timing_desc_init_f9d0
/// Publish one 16-byte timing descriptor to its static slot.
///
/// The original writes four words to the global at `0x0110F220`: a code
/// pointer, the word from its own unwritten stack slot (reproduced here as
/// the checker's defined stack fill), zero, and a second code pointer. Both
/// code pointers carry HIGHLOW fixups and are relocated. The function never
/// writes EAX, so exit EAX is entry EAX passed through; that passthrough is
/// proven by disassembly rather than differentially (a `cdecl/0` rewrite
/// cannot observe entry EAX), hence the contract compares no return value.
lf_checker_rt::export!(cdecl, rw_00e5f9d0() -> u32 {
    unsafe {
        /// Descriptor slot this instance publishes (file VA).
        const SLOT: u32 = 0x0110F220;
        /// First code pointer stored (file VA, relocated).
        const FIRST: u32 = 0x0043EA90;
        /// Second code pointer stored (file VA, relocated).
        const SECOND: u32 = 0x00409610;
        /// Checker's defined fill for the unwritten stack slot (contract `stack_fill`).
        const STACK_FILL: u32 = 0;
        let slot = lf_checker_rt::global::<u32>(SLOT);
        slot.write(lf_checker_rt::relocated(FIRST));
        slot.add(1).write(STACK_FILL);
        slot.add(2).write(0);
        slot.add(3).write(lf_checker_rt::relocated(SECOND));
        0
    }
});
