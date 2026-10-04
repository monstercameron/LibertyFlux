// original: 0x008aa080 audio_slot_alloc
/// Allocate the first free voice slot for a value.
///
/// Scans the 0x258 dwords at `this+0x28a8` for the first zero entry and
/// stores the argument there. A zero argument, or a full table, stores
/// nothing. Returns nothing; exit EAX depends on entry EAX on one path, so
/// the contract compares no return channel.
export!(thiscall, rw_008aa080(this: *mut u8, value: u32) -> () {
    unsafe {
        if value == 0 {
            return;
        }
        let slots = this.add(0x28a8) as *mut u32;
        for i in 0..0x258usize {
            if *slots.add(i) == 0 {
                *slots.add(i) = value;
                break;
            }
        }
    }
});
