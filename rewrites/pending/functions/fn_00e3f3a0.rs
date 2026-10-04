// original: 0x00e3f3a0 MediaTrack_Restart
// 0x00E3F3A0: restart a media track: drop the current stream, rebuild the
// reader, and either rebind a plain source or open a new stream through
// the factory. Returns 1 when a stream is open. (thiscall/1, AL-only)
export!(thiscall, rw_00e3f3a0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let old = *(this.add(0x3b4) as *mut u32);
        if old != 0 {
            callee_thiscall!(1, u32, old);
            let pending = *((old.wrapping_add(0x1e)) as *const u16);
            if pending != 0 {
                let queue = *((old.wrapping_add(0x18)) as *const u32);
                callee_stdcall!(2, u32, queue, pending as u32);
            }
            callee_cdecl!(3, u32, old);
            *(this.add(0x3b4) as *mut u32) = 0;
        }
        let reader = (this as u32).wrapping_add(0x3b8);
        callee_cdecl!(4, u32, reader, 0);
        *((reader.wrapping_add(4)) as *mut u32) = 0;
        let kind = *(this.add(0x384) as *const u32);
        if kind == 8 || kind == 0x0b || kind == 0x10 {
            callee_thiscall!(6, u32, (this as u32).wrapping_add(0x3d0));
            return 0;
        }
        let stream = callee_cdecl!(
            5, u32,
            *(this.add(0x395)) as u32,
            kind,
            arg,
            0,
            *(this.add(0x390) as *const u32),
            *(this.add(0x396)) as u32,
            *(this.add(0x397)) as u32,
            0
        );
        *(this.add(0x3b4) as *mut u32) = stream;
        if stream == 0 {
            return 0;
        }
        let stamp = (*global::<u32>(0x1173594)).wrapping_add(0x2BF20);
        *(this.add(0x3b0) as *mut u32) = stamp;
        1
    }
});
