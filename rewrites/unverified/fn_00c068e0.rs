// original: 0x00c068e0 stream_slot_alloc_init
/// Allocate a slot through the grow helper and initialise it from `arg`.
///
/// Calls the grow helper (thiscall/1) with size 0x10, then the slot-init
/// helper (thiscall/1) on the answer with `arg`, and returns the answer.
/// Thiscall: one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c068e0(this: u32, arg: u32) -> u32 {
    unsafe {
        const GROW: u32 = 1;
        const INIT: u32 = 2;
        let slot: u32 = lf_checker_rt::callee_thiscall!(GROW, u32, this, 0x10);
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, slot, arg);
        slot
    }
});
