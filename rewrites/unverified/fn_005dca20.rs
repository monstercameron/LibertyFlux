// original: 0x005DCA20 initialize_ped_task_streams

/// Initialize the task streaming state used by the pedestrian task subsystem.
///
/// The initializer requests one named stream, resolves its identifier, and
/// passes the resolved values through the lookup and activation helpers. It
/// then releases any current stream objects in the two global slots and
/// assigns the two fixed task stream names to those objects. The calls are
/// unconditional and occur in this order. The final tail call performs the
/// subsystem's shared release step. This is a cdecl, no-argument initializer;
/// its ABI return register has no semantic value.
lf_checker_rt::export!(cdecl, rw_005dca20() -> () {
    const REQUESTED_STREAM_NAME: u32 = 0x00F917D8;
    const FIRST_TASK_STREAM_SLOT_ADDRESS: u32 = 0x010344D4;
    const SECOND_TASK_STREAM_SLOT_ADDRESS: u32 = 0x010344D0;
    const FIRST_TASK_ASSET_NAME: u32 = 0x00F917AC;
    const SECOND_TASK_ASSET_NAME: u32 = 0x00F917BC;

    let _ = lf_checker_rt::callee_cdecl!(1, u32,);

    let requested_stream = lf_checker_rt::callee_cdecl!(
        2,
        u32,
        lf_checker_rt::relocated(REQUESTED_STREAM_NAME),
    );
    let resolved_stream = lf_checker_rt::callee_cdecl!(3, u32, requested_stream);
    let _ = lf_checker_rt::callee_cdecl!(4, u32, resolved_stream);

    let first_stream_slot = lf_checker_rt::relocated(FIRST_TASK_STREAM_SLOT_ADDRESS);
    let second_stream_slot = lf_checker_rt::relocated(SECOND_TASK_STREAM_SLOT_ADDRESS);
    let _ = lf_checker_rt::callee_thiscall!(5, u32, first_stream_slot);
    let _ = lf_checker_rt::callee_thiscall!(6, u32, second_stream_slot);
    let _ = lf_checker_rt::callee_thiscall!(
        7,
        u32,
        first_stream_slot,
        lf_checker_rt::relocated(FIRST_TASK_ASSET_NAME),
    );
    let _ = lf_checker_rt::callee_thiscall!(
        8,
        u32,
        second_stream_slot,
        lf_checker_rt::relocated(SECOND_TASK_ASSET_NAME),
    );
    let _ = lf_checker_rt::callee_cdecl!(9, u32,);
});
