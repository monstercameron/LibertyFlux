// original: 0x008a3c30 rage::audSwitchSound::audSwitchSound_2
/// Scalar (non-deleting) destructor body of `audSwitchSound`.
///
/// Original 0x008A3C30 (`thiscall/0`, tail-calls the base destructor):
/// installs the base vtable, releases the voice like rw_008a37f0, then
/// releases the switch slot selected by the byte at +0xF0 (unless 0xFF)
/// and clears it, before tail-calling the base destructor.
export!(thiscall, rw_008a3c30(this: u32) -> u32 {    unsafe {
        (this as *mut u32).write_unaligned(relocated(0xe7ae34));
    }
    let no_release = unsafe { ((this + 0x39) as *const u8).read_unaligned() } & 0x40 != 0;
    if !no_release {
        let id = unsafe { ((this + 0x48) as *const u8).read_unaligned() } as u32;
        if id != 0xFF {
            let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
            if ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(id))
    }) != 0 {
                callee_thiscall!(1, u32, this, 0);
            }
        }
    }
    let sel = unsafe { ((this + 0xf0) as *const u8).read_unaligned() } as u32;
    if sel != 0xFF {
        let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
        let slot = ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(sel))
    });
        callee_thiscall!(2, u32, relocated(0x115d8a0), slot, bank);
        unsafe {
            ((this + 0xf0) as *mut u8).write_unaligned(0xFF);
        }
    }
    callee_thiscall!(3, u32, this)
});
