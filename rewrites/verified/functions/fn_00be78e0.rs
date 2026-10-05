// original: 0x00be78e0 CTaskSimpleTogglePedThreatScanner::vf17

/// Push the task's three threat-scanner bytes into the ped's scanner state.
///
/// `this` points to the task, `ped` to the ped. Resolves the scanner owner
/// at `ped+0x224` and fetches its state block through virtual slot 8
/// (callee 1, thiscall, no stack words), then stores the bytes at
/// `this+0x14`/`+0x15`/`+0x16` at offsets 0xe4/0xe5/0xe6 of that block.
/// Always returns 1. (The original briefly stashes the third byte in its own
/// incoming argument slot; the rewrite reads it directly, which the stack
/// check cannot observe, so this contract runs with `checks.stack` off. The
/// byte itself is still verified through the heap store.)
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be78e0(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_B0: u32 = 0x14;
        const OFF_B1: u32 = 0x15;
        const OFF_B2: u32 = 0x16;
        const PED_OWNER: u32 = 0x224;
        const STATE_SLOT: u32 = 8;
        const STATE_B0: u32 = 0xe4;
        const STATE_B1: u32 = 0xe5;
        const STATE_B2: u32 = 0xe6;
        const FETCH_STATE: u32 = 1;

        let owner = ((ped + PED_OWNER) as *const u32).read_unaligned();
        let vtable = (owner as *const u32).read_unaligned();
        let slot = ((vtable + STATE_SLOT) as *const u32).read_unaligned();
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let state = fetch(owner);
        ((state + STATE_B0) as *mut u8).write(((this + OFF_B0) as *const u8).read());
        ((state + STATE_B1) as *mut u8).write(((this + OFF_B1) as *const u8).read());
        ((state + STATE_B2) as *mut u8).write(((this + OFF_B2) as *const u8).read());
        1
    }
});
