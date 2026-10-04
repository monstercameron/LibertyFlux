// original: 0x009f50c0 table_forward_notify_24
/// Forward a table word to the resolver, stamp the shared tick, notify.
export!(cdecl, rw_009f50c0() -> u32 {
    // SAFETY: worker maps the original image; all globals are .data.
    unsafe {
        callee_cdecl!(1, u32, global::<u32>(0x012F_9F78).read());
        let stamp = global::<u32>(0x0117_35B4).read();
        global::<u32>(0x012B_6230).write(stamp);
        callee_cdecl!(2, u32, 0x18)
    }
});
