// original: 0x0099dda0 speech_voice_ensure
/// Ensures the voice slot of a speech audio entity is populated.
///
/// Returns at once when the slot at offset 0x9c is already set. Otherwise it
/// inspects two flag bytes of the linked object at offset 8: unless the first
/// is set and the second is clear while callee 1 also answers zero, it takes
/// the acquisition path, chaining callees 2 and 3 and, on a zero answer,
/// probing the current slot through callee 5 and storing a fallback global
/// on a zero answer. The quiet path stores a default global into the slot.
/// Only low bytes of the callee 1 and 5 answers are significant. Callee 2
/// takes two stack words (a one on top of the argument); the stub pops both,
/// which is the only cleanup that keeps the original's own pops and return
/// aligned.
export!(thiscall, rw_0099dda0(this: u32, a0: u32) -> () {
    unsafe {
        const MANAGER: u32 = 0x1288780;
        const DEFAULT_VOICE: u32 = 0x1284390;
        const FALLBACK_VOICE: u32 = 0x12844b4;
        if *((this.wrapping_add(0x9c)) as *const u32) != 0 {
            return;
        }
        let inner = *((this.wrapping_add(8)) as *const u32);
        let take_call_path = if *((inner.wrapping_add(0x218)) as *const u8) != 0 {
            true
        } else if *((inner.wrapping_add(0x219)) as *const u8) == 0 {
            true
        } else {
            (callee_thiscall!(1, u32, this) as u8) != 0
        };
        if !take_call_path {
            *(this.wrapping_add(0x9c) as *mut u32) = *global::<u32>(DEFAULT_VOICE);
            return;
        }
        let r2 = callee_thiscall!(2, u32, this, 1, a0);
        let r3 = callee_thiscall!(3, u32, relocated(MANAGER), r2);
        if r3 != 0 {
            callee_thiscall!(4, u32, this, r3);
            return;
        }
        let cur = *((this.wrapping_add(0x9c)) as *const u32);
        if (callee_cdecl!(5, u32, cur) as u8) == 0 {
            *(this.wrapping_add(0x9c) as *mut u32) = *global::<u32>(FALLBACK_VOICE);
        }
    }
});
