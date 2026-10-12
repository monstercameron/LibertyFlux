// original: 0x0098b8c0 audEntityRadioEmitter::matches_channel
/// Returns one when the requested channel matches the emitter's stored channel.
///
/// The receiver stores the channel identifier as a 32-bit word at byte offset
/// four. The method compares the full word and returns a zero-extended boolean;
/// its single stack argument is removed by the callee.
const CHANNEL_ID: u32 = 4;

export!(thiscall, rw_0098b8c0(this_: u32, requested_channel: u32) -> u32 {
    let current_channel = unsafe {
        (this_.wrapping_add(CHANNEL_ID) as *const u32).read()
    };
    u32::from(current_channel == requested_channel)
});
