// original: 0x00e61f20 timer_param_store
// Parameter fetch-and-store: calls the shared worker reached through the
// data-table slot with a fixed four-word request, publishes the answer in
// the output slot, and returns the answer.
lf_checker_rt::export!(cdecl, rw_00e61f20() -> u32 {
    const WORKER_SLOT: u32 = 0x00E73194;
    const OUT_SLOT: u32 = 0x01B48FE0;
    const REQUEST: u32 = 0x7FFF;
    let worker: extern "stdcall" fn(u32, u32, u32, u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::global::<u32>(WORKER_SLOT).read() as usize)
    };
    // Push order in the original is 0, REQUEST, 0, 0, so the last push
    // lands at [esp+4]: arg0=0, arg1=0, arg2=REQUEST, arg3=0.
    let answer = worker(0, 0, REQUEST, 0);
    unsafe { lf_checker_rt::global::<u32>(OUT_SLOT).write(answer); }
    answer
});
