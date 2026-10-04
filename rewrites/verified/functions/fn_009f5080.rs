// original: 0x009f5080 table_forward_notify_26
/// Forward a table word to the resolver, stamp the shared tick, notify.
export!(cdecl, rw_009f5080() -> u32 {
    // SAFETY: worker maps the original image; all globals are .data.
    unsafe {
        callee_cdecl!(1, u32, global::<u32>(0x012F_A008).read());
        let stamp = global::<u32>(0x0117_35B4).read();
        global::<u32>(0x012B_6238).write(stamp);
        callee_cdecl!(2, u32, 0x1a)
    }
});
