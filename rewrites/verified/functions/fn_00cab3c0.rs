// original: 0x00CAB3C0 event_dual_construct (proposed)

/// Construct a dual-word event record in place and return its address.
///
/// Runs the base constructor on `this`, installs the record's virtual table,
/// stores the low words of the third and first stack arguments at `+0x14`
/// and `+0x18`, and clears flag bit 1 at `+0x28`. Then it asks the state
/// source for a state block: a null source, or a null answer, stores zero at
/// `+0x24`, otherwise the answer is initialised and stored there. Finally
/// the worker is called with (second argument, fourth and fifth arguments as
/// bit patterns, 1). All float movement is bitwise.
///
/// Original: 0x00CAB3C0 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00cab3c0(this: u32, a0: u32, a1: u32, a2: u32, f3_bits: u32, f4_bits: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const SOURCE: u32 = 2;
        const INIT: u32 = 3;
        const WORKER: u32 = 4;
        const VTABLE: u32 = 0x00ED8974;
        const STATE_GLOBAL: u32 = 0x0179D110;
        const WORD_HI: u32 = 0x14;
        const WORD_LO: u32 = 0x18;
        const STATE: u32 = 0x24;
        const FLAGS: u32 = 0x28;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        let flags = (this.wrapping_add(FLAGS) as *const u32).read_unaligned();
        (this.wrapping_add(FLAGS) as *mut u32).write_unaligned(flags & 0xFFFF_FFFD);
        (this.wrapping_add(WORD_HI) as *mut u16).write_unaligned((a2 & 0xFFFF) as u16);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(WORD_LO) as *mut u16).write_unaligned((a0 & 0xFFFF) as u16);
        let source = lf_checker_rt::global::<u32>(STATE_GLOBAL).read_unaligned();
        let handle = lf_checker_rt::callee_thiscall!(SOURCE, u32, source);
        let state = if handle == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(INIT, u32, handle)
        };
        (this.wrapping_add(STATE) as *mut u32).write_unaligned(state);
        lf_checker_rt::callee_thiscall!(WORKER, u32, this, a1, f3_bits, f4_bits, 1);
        this
    }
});
