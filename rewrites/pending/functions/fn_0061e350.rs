// original: 0x0061e350 rage::snEventAddingGamer::AutoIdDesc__::AutoIdDesc__
/// Build and register the static auto-id descriptor for the `adding_gamer` network event.
///
/// The original fills the static descriptor node (flag byte, constant header
/// words, descriptor id 7 and two relocated table pointers), asks the event
/// registry for its head pointer (intercepted by the checker), links the node at
/// the head of the list, bumps the registry counters, installs the final vtable
/// and returns the node address.
export!(cdecl, rw_0061e350() -> u32 {
    unsafe {
        /// Static descriptor node this function initialises (file VA).
        const NODE: u32 = 0x019F07A0;
        /// Auto-assigned event id stored in the descriptor.
        const EVENT_ID: u32 = 7;
        /// Per-event descriptor table pointer (file VA).
        const DESCRIPTOR: u32 = 0x00FE202C;
        /// Shared table pointer, same for every event (file VA).
        const SHARED: u32 = 0x019F0800;
        /// Vtable installed while the node is being registered (file VA).
        const VTABLE_LINKING: u32 = 0x00FE2264;
        /// Vtable installed once the node is linked (file VA).
        const VTABLE_FINAL: u32 = 0x00FE2314;
        let node = relocated(NODE);
        let flag = global::<u8>(NODE + 0x1C);
        *flag |= 1;
        let w = global::<u32>(NODE);
        *w.add(1) = 1;
        *w.add(2) = EVENT_ID;
        *w.add(4) = relocated(DESCRIPTOR);
        *w.add(5) = relocated(SHARED);
        *w.add(6) = 0;
        *w = relocated(VTABLE_LINKING);
        let head = callee_cdecl!(1, u32,);
        let hp = head as *mut u32;
        *w.add(6) = *hp;
        *hp = node;
        if *flag & 1 != 0 {
            let want = *w.add(2);
            if want > *(hp.add(4)) {
                *(hp.add(4)) = want.wrapping_add(1);
            }
            *(hp.add(3)) = (*(hp.add(3))).wrapping_add(1);
        }
        *(hp.add(2)) = (*(hp.add(2))).wrapping_add(1);
        *w = relocated(VTABLE_FINAL);
        node
    }
});
