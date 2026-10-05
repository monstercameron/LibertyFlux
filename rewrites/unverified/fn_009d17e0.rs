// original: 0x009D17E0 stream_lazy_dispatch (proposed)
//
/// Lazily initialises streaming once, then dispatches on a flag pair.
///
/// Unless the init flag at `0x012958B0` is set (bit 0), sets it and runs the
/// creator (callee 1) and the seeder (callee 2 with `0xE72000`); both returns
/// are ignored. A null context global (`0x012958AC`) returns null. When the
/// context's first byte is nonzero while the argument's low byte is zero,
/// the context is returned as is; otherwise a key pair is fetched (callee 3 into
/// a frame slot and the incoming arg slot, which the original reuses as
/// scratch; the rewrite keeps a local instead, so the stack-word check is off
/// and the keys are observed through what the next call consumes) and
/// dispatched with (callee 4 on the context), and the context is returned.
/// Stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_009D17E0(arg: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x012958B0;
        const CTX: u32 = 0x012958AC;
        const SEED: u32 = 0xe72000;
        const CREATE: u32 = 1;
        const SEEDER: u32 = 2;
        const FETCH: u32 = 3;
        const DISPATCH: u32 = 4;
        let flag = lf_checker_rt::global::<u32>(FLAG);
        if flag.read() & 1 == 0 {
            flag.write(flag.read() | 1);
            let _: u32 = lf_checker_rt::callee_cdecl!(CREATE, u32,);
            let _: u32 = lf_checker_rt::callee_cdecl!(SEEDER, u32, lf_checker_rt::relocated(SEED));
        }
        let ctx = lf_checker_rt::global::<u32>(CTX).read();
        if ctx == 0 {
            return 0;
        }
        let live = (ctx as *const u8).read();
        if live != 0 && (arg as u8) == 0 {
            return ctx;
        }
        let mut k0 = 0u32;
        let mut k1 = arg;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            FETCH, u32, (&mut k0 as *mut u32) as u32, (&mut k1 as *mut u32) as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(DISPATCH, u32, ctx, k0, k1);
        ctx
    }
});
