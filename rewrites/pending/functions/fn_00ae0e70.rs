// original: 0x00ae0e70 fetch_active_view
//
// Thunk: loads the active object from its global slot and tail-jumps to
// the shared worker, forwarding the result. Takes no stack arguments
// (Verified: sole caller issues adjacent calls with no pushes).
use lf_checker_rt::{callee_thiscall, export, global};

const G_ACTIVE: u32 = 0x01593B6C;
const W_WORKER: u32 = 1; // 0xD6C650 thiscall/0: shared worker (tail target)

export!(cdecl, rw_ae0e70() -> u32 {
    // SAFETY: global word fabricated by the contract.
    let obj = unsafe { global::<u32>(G_ACTIVE).read() };
    callee_thiscall!(W_WORKER, u32, obj)
});
