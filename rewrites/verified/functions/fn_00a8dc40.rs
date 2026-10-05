// original: 0x00a8dc40 pool_forward_lookup (proposed)

/// Look up the handler for this instance's kind, tail-calling into it.
///
/// `this` carries a signed 16-bit kind at +0x2E; the handler table gives
/// the object and its word at +0x70 becomes the new object for the tail
/// call, with `key` forwarded. Returns whatever the handler returns.
///
/// Original: 0x00A8DC40 (thiscall, one stack word, tail-calls its callee;
/// the bytes past its jump are padding and the next function).
lf_checker_rt::export!(thiscall, rw_00a8dc40(this: u32, key: u32) -> u32 {
    unsafe {
        const CALLEE_HANDLER: u32 = 1;
        const KIND: u32 = 0x2e;
        const HANDLER_TABLE: u32 = 0x1295cd8;
        const HANDLER_THIS: u32 = 0x70;
        let kind =
            ((this + KIND) as *const i16).read_unaligned() as i32 as usize;
        let entry =
            lf_checker_rt::global::<u32>(HANDLER_TABLE).add(kind).read_unaligned();
        let target = ((entry + HANDLER_THIS) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CALLEE_HANDLER, u32, target, key)
    }
});
