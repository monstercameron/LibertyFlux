// original: 0x00a9c7f0 stream_shutdown (proposed)

/// Shut the streaming subsystem down: stop the workers, release every node
/// twice, and tail into the final teardown.
///
/// `this` is the subsystem object. The two worker threads are stopped, then
/// the mode selector runs with (1, min(1, `[G1]`)) where G1 is the dword at
/// file address 0x0106b310 (the lesser value wins, compared signed). The
/// notifier at file address 0x00431940 runs with the context at file
/// address 0x017f583c in ECX, the selector runs twice more with (2, 0) and
/// (0, 2), and then the node list at `this + 0x8ec50` is walked twice
/// (link at node `+0`), releasing each node with pass 0 and then with pass
/// 1, with a final selector call (0, 1) between the walks. The post-pass
/// hook and a second worker stop follow, and the teardown routine is
/// tail-called with no arguments; its answer is the result.
///
/// Original: 0x00a9c7f0 (thiscall, no stack arguments; ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_00a9c7f0(this: u32) -> u32 {
    unsafe {
        const LEVEL_GLOBAL: u32 = 0x0106b310;
        const CONTEXT_GLOBAL: u32 = 0x017f583c;
        const NOTIFY_TARGET: u32 = 0x01110090;
        const LIST_OFF: u32 = 0x8ec50;
        const STOP_WORKER: u32 = 1;
        const STOP_HELPER: u32 = 2;
        const SELECT: u32 = 3;
        const NOTIFY: u32 = 4;
        const RELEASE_NODE: u32 = 5;
        const POST_PASS: u32 = 6;
        const TEARDOWN: u32 = 7;
        lf_checker_rt::callee_thiscall!(STOP_WORKER, u32, this);
        lf_checker_rt::callee_thiscall!(STOP_HELPER, u32, this);
        let level = lf_checker_rt::global::<i32>(LEVEL_GLOBAL).read_unaligned();
        let first = if level < 1 { level as u32 } else { 1 };
        lf_checker_rt::callee_cdecl!(SELECT, u32, 1, first);
        let ctx = lf_checker_rt::global::<u32>(CONTEXT_GLOBAL).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            NOTIFY,
            u32,
            ctx,
            lf_checker_rt::relocated(NOTIFY_TARGET)
        );
        lf_checker_rt::callee_cdecl!(SELECT, u32, 2, 0);
        lf_checker_rt::callee_cdecl!(SELECT, u32, 0, 2);
        for pass in [0u32, 1] {
            if pass == 1 {
                lf_checker_rt::callee_cdecl!(SELECT, u32, 0, 1);
            }
            let mut node = ((this + LIST_OFF) as *const u32).read_unaligned();
            while node != 0 {
                lf_checker_rt::callee_thiscall!(RELEASE_NODE, u32, node, pass);
                node = (node as *const u32).read_unaligned();
            }
        }
        lf_checker_rt::callee_thiscall!(POST_PASS, u32, this);
        lf_checker_rt::callee_thiscall!(STOP_WORKER, u32, this);
        lf_checker_rt::callee_cdecl!(TEARDOWN, u32,)
    }
});
