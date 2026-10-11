// original: 0x008E4970 frontend_audio_event_update

/// Update one frontend audio event from its event record and stop flag.
///
/// `this` is the owning frontend audio entity. The event record is a pointer
/// in the first stack argument; its first byte gates all work. When the third
/// stack argument is zero, the entity's sound handle at `+0x484` receives the
/// current scalar value from `+0x48c`. When the flag is nonzero, `+0x488` is
/// cleared, the event-kind field at `+0x470` is set to 5, the sound handle is
/// reset with `-1`, and the event record is submitted to the creation helper;
/// its result is stored at `+0x474`.
///
/// All three helpers are intercepted by the checker. The method uses the
/// 32-bit thiscall convention with four stack words and does not define a
/// semantic return value.
lf_checker_rt::export!(thiscall, rw_008e4970(this: u32, event: u32, _unused: u32, stop: u32, _reserved: u32) -> u32 {
    unsafe {
        const SOUND_HANDLE: u32 = 0x484;
        const STORED_VOLUME: u32 = 0x48c;
        const ACTIVE_EVENT: u32 = 0x488;
        const EVENT_KIND: u32 = 0x470;
        const CREATED_EVENT: u32 = 0x474;

        if (event as *const u8).read() == 0 {
            return 0;
        }

        let sound = (this.wrapping_add(SOUND_HANDLE) as *const u32).read_unaligned();
        if stop == 0 {
            let volume = (this.wrapping_add(STORED_VOLUME) as *const f32).read_unaligned().to_bits();
            let _ = lf_checker_rt::callee_thiscall!(1, u32, sound, volume);
            return 0;
        }

        (this.wrapping_add(ACTIVE_EVENT) as *mut u32).write_unaligned(0);
        (this.wrapping_add(EVENT_KIND) as *mut u32).write_unaligned(5);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, sound, 0);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, u32::MAX);
        let created = lf_checker_rt::callee_cdecl!(3, u32, event, 0);
        (this.wrapping_add(CREATED_EVENT) as *mut u32).write_unaligned(created);
        0
    }
});
