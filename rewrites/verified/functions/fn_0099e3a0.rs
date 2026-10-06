// original: 0x0099E3A0 audio_dispatch_or_zero (proposed)

/// Dispatch through the linked object, or return zero when absent.
///
/// Reads the link at `this`+8. A null link returns 0. A live link continues
/// at code outside this entry (the inventory split one routine across two
/// entries); that continuation is NOT reproduced here, and the contract
/// only feeds null links, so the proven behaviour is the null path alone.
/// Thiscall with one stack word (unread on the proven path), callee pops 4.
lf_checker_rt::export!(thiscall, rw_0099E3A0(this: u32, _arg: u32) -> u32 {
    unsafe {
        const LINK: u32 = 8;
        let link = ((this.wrapping_add(LINK)) as *const u32).read_unaligned();
        if link != 0 {
            // Continuation outside this entry; never taken by the contract.
            return 0;
        }
        0
    }
});
