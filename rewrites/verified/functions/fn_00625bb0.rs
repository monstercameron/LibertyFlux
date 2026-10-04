// original: 0x00625bb0 gated_capability_check
// Run a chain of gated capability checks over two objects.
//
// Probes the peer with the global context, verifies both identity words,
// requires at least one of them set, then validates the tail block. Any
// failed step returns 0; all steps passing returns 1.
export!(thiscall, rw_00625bb0(obj: u32, other: u32) -> u32 {
    unsafe {
        const CONTEXT_SLOT: u32 = 0x019f087c;
        const VERIFY_TAG: u32 = 0x20;
        const TAIL_OFF: u32 = 8;
        let ctx: u32 = *(global::<u32>(CONTEXT_SLOT) as *const u32);
        if callee_fastcall!(1, u32, ctx, other) & 0xff == 0 {
            return 0;
        }
        let first: u32 = *(obj as *const u32);
        let second: u32 = *((obj.wrapping_add(4)) as *const u32);
        if callee_thiscall!(2, u32, other, first, VERIFY_TAG) & 0xff == 0 {
            return 0;
        }
        if callee_thiscall!(2, u32, other, second, VERIFY_TAG) & 0xff == 0 {
            return 0;
        }
        if first | second == 0 {
            return 0;
        }
        if callee_thiscall!(3, u32, other, obj.wrapping_add(TAIL_OFF)) & 0xff == 0 {
            return 0;
        }
        1
    }
});
