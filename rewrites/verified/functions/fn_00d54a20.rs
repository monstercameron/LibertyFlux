// original: 0x00d54a20 ccam_busted_init

/// Initialise the busted camera from the global tick and register it.
///
/// `this` points to the object. The global tick `TICK` is copied to `STAMP`,
/// `GUARD` is zeroed, the shared mode fetch runs (its answer lands in
/// `MODE`), the shared member setup runs with `this` in ECX, and the object
/// at `PEER` is registered with arguments (0, 3). Returns 1 in AL (upper EAX
/// passes through, so only AL is compared).
///
/// Original: 0x00d54a20 (thiscall, no stack arguments, three calls).
lf_checker_rt::export!(thiscall, rw_00d54a20(this: u32) -> u32 {
    unsafe {
        /// Global tick copied into the object.
        const TICK: u32 = 0x011735c4;
        /// Stamp slot receiving the tick.
        const STAMP: u32 = 0x144;
        /// Guard word zeroed on entry.
        const GUARD: u32 = 0x148;
        /// Slot receiving the mode fetch answer.
        const MODE: u32 = 0x140;
        /// Slot holding the peer object to register.
        const PEER: u32 = 0x12c;
        /// Shared mode fetch (intercepted; thiscall, no arguments).
        const FETCH_MODE: u32 = 1;
        /// Shared member setup (intercepted; thiscall, no arguments).
        const MEMBER_SETUP: u32 = 2;
        /// Peer registration (intercepted; thiscall, two stack arguments).
        const REGISTER: u32 = 3;
        let tick = (lf_checker_rt::global::<u32>(TICK) as *const u32).read();
        ((this + STAMP) as *mut u32).write_unaligned(tick);
        ((this + GUARD) as *mut u32).write_unaligned(0);
        let mode = lf_checker_rt::callee_thiscall!(FETCH_MODE, u32, this);
        ((this + MODE) as *mut u32).write_unaligned(mode);
        lf_checker_rt::callee_thiscall!(MEMBER_SETUP, u32, this);
        let peer = ((this + PEER) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(REGISTER, u32, peer, 0, 3);
        1
    }
});
