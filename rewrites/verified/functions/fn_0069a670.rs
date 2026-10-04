// original: 0x0069a670 guarded_forward_to_69b820
/// Null-guarded forward to the channel initializer: calls it with `obj` both
/// as `this` and as the pushed argument, returning its answer. A null pointer
/// makes no call; the return channel is unchecked because that path keeps
/// entry EAX.
export!(cdecl, rw_0069a670(obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            0
        } else {
            callee_thiscall!(1, u32, obj, obj)
        }
    }
});
