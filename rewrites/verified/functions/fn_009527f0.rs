// original: 0x009527f0 channel_marker_search
/// Searches the channel buffers for the wanted marker and reports a position.
///
/// A probe call seeds a counter that bumps when the current word still
/// matches the probe answer. Starting from the channel named by the fetched
/// descriptor, each active channel's buffer is walked: zero ends the buffer,
/// 5 counts a separator, and a 0x1d byte followed by the wanted word returns
/// the descriptor's position word minus the base. Other bytes advance by a
/// reported width. Falling off the channel table returns the current word
/// minus the base.
export!(cdecl, rw_009527f0(wanted: u32) -> u32 {
    unsafe {
        const CURRENT: u32 = 0x011F_702C;
        const BASE: u32 = 0x011F_7028;
        const COUNTER0: u32 = 0x011F_70C4;
        const BUFFERS: u32 = 0x011F_6F7C;
        const ACTIVE: u32 = 0x011F_6FF0;
        const COUNT: u32 = 0x011F_6FFB;
        const SEPARATOR: u8 = 5;
        const MARKER: u8 = 0x1D;
        const POS_OFF: u32 = 0xC;
        let probe = callee_cdecl!(1, u32,);
        let mut counter = *global::<u32>(COUNTER0);
        if *global::<u32>(CURRENT) == probe {
            counter = counter.wrapping_add(1);
        }
        let desc = callee_cdecl!(2, u32, counter);
        let mut off = *(desc as *const u32);
        let mut ch = *((desc.wrapping_add(8)) as *const u8);
        let buf = *((relocated(BUFFERS).wrapping_add((ch as u32).wrapping_mul(4)))
            as *const u32);
        let count = *global::<u8>(COUNT);
        loop {
            if *((relocated(ACTIVE).wrapping_add(ch as u32)) as *const u8) != 0 {
                loop {
                    let b = *((buf.wrapping_add(off)) as *const u8);
                    if b == 0 {
                        break;
                    }
                    if b == SEPARATOR {
                        counter = counter.wrapping_add(1);
                    } else if b == MARKER
                        && *((buf.wrapping_add(off).wrapping_add(4)) as *const u32)
                            == wanted
                    {
                        let found = callee_cdecl!(2, u32, counter);
                        return (*((found.wrapping_add(POS_OFF)) as *const u32))
                            .wrapping_sub(*global::<u32>(BASE));
                    }
                    off = off.wrapping_add(callee_cdecl!(3, u32, b as u32));
                }
            }
            ch = ch.wrapping_add(1);
            if ch >= count {
                break;
            }
            off = 0;
        }
        (*global::<u32>(CURRENT)).wrapping_sub(*global::<u32>(BASE))
    }
});
