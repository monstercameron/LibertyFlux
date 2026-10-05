// original: 0x00b951e0 STOP_CREDITS
/// Script native `STOP_CREDITS` (hash 0x4F0F2AA8).
///
/// A five-byte jump to a shared handler implementation. The rewrite
/// forwards the context pointer through the checker's tail-call stub,
/// which logs and answers exactly like the jumped-to code. The worker
/// patches the jump to a call, so on the original side the stub sees the
/// trampoline return address as arg 0 and the context pointer as arg 1;
/// the rewrite passes a dummy word plus the context pointer, arg 0 is
/// skipped (a patching artifact, not behaviour), and the forwarded
/// context pointer itself is compared exactly.
export!(cdecl, rw_00b951e0(ctx: *const u8) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, 0, ctx as u32)
    }
});
