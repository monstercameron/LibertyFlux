// original: 0x0059e060 dispatch_state_handler
// Walk a three-link state chain and tail-dispatch through its handler slot.
//
// Each link is null-checked (a null link returns 0); the live chain ends in
// a computed jump through `*(*(c) + 0x1C)`. A null base faults on both sides
// (fault parity, not a FALSE return). Only AL carries the boolean: the
// third null check leaves the stale link address above AL, so the contract
// compares the `al` channel.
export!(cdecl, rw_0059E060() -> u32 {
    unsafe {
        let o1 = *global::<u32>(0x18B6C98);
        let o1v = *((o1 + 0x1E4) as *const u32);
        if o1v == 0 {
            return 0;
        }
        let o2 = *((o1v + 0x1E0) as *const u32);
        if o2 == 0 {
            return 0;
        }
        let c = *((o2 + 0x1E0) as *const u32);
        if c == 0 {
            return 0;
        }
        let v = *(c as *const u32);
        let target = *((v + 0x1C) as *const u32);
        let f: extern "cdecl" fn() -> u32 = std::mem::transmute(target as usize);
        f()
    }
});
