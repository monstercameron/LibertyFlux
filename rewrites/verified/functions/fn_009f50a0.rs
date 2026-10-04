// original: 0x009f50a0 table_forward_notify_25
/// Forward a table word to the resolver, stamp the shared tick, notify.
export!(cdecl, rw_009f50a0() -> u32 {
    // SAFETY: worker maps the original image; all globals are .data.
    unsafe {
        callee_cdecl!(1, u32, global::<u32>(0x012F_9F6C).read());
        let stamp = global::<u32>(0x0117_35B4).read();
        global::<u32>(0x012B_6234).write(stamp);
        callee_cdecl!(2, u32, 0x19)
    }
});
