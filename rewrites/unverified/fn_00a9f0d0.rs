// original: 0x00A9F0D0 loader_dispatch_update
/// Runs the current object's update hook, then tail-forwards to the update
/// method on the streaming object stored in the shared global slot. The
/// forwarded method takes no stack arguments; its return value is the result
/// of this wrapper.
lf_checker_rt::export!(thiscall, rw_00A9F0D0(this: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let shared_stream = lf_checker_rt::global::<u32>(0x1bb6674).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, shared_stream)
    }
});
