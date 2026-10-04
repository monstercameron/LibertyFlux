// original: 0x00946af0 radio_set_station_mode
use lf_checker_rt::{callee_thiscall, export};

/// Store a station's mode bytes and refresh its two channel records.
///
/// Records the two mode bytes and the optional channel-pointer pair, then
/// refreshes the active channel record: when the record's threshold check
/// passes (or no mode byte is set) the record is reconfigured, otherwise it
/// is reset. The alternate record is reconfigured when active. (Original
/// 0x00946AF0. Callers ignore the return value.)
export!(thiscall, rw_00946af0(station: u32, mode_a: u32, channels: u32, mode_b: u32) -> u32 {
    const MODE_A_OFF: u32 = 0x191B;
    const MODE_B_OFF: u32 = 0x191A;
    const CHAN_OFF: u32 = 0x17A0;
    const INDEX_OFF: u32 = 0x1917;
    const FLAG_OFF: u32 = 0x191C;
    const RECORD_STRIDE: u32 = 0xBD0;
    const ACTIVE_OFF: u32 = 0xBCA;
    const ADJUST_OFF: u32 = 0xBCC;
    const LEVEL_OFF: u32 = 0x994;
    const BASE_OFF: u32 = 0xBBC;
    const LIMIT_OFF: u32 = 0x990;
    const LEVEL_BIAS: u32 = 0x1B58;
    unsafe {
        ((station + MODE_A_OFF) as *mut u8).write(mode_a as u8);
        ((station + MODE_B_OFF) as *mut u8).write(mode_b as u8);
        let slots = (station + CHAN_OFF) as *mut u32;
        if channels == 0 {
            slots.write(0);
            slots.add(1).write(0);
        } else {
            slots.write((channels as *const u32).read());
            slots.add(1).write(((channels + 4) as *const u32).read());
        }
        let index = ((station + INDEX_OFF) as *const u8).read() as u32;
        let active = station.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        if ((active + ACTIVE_OFF) as *const u8).read() != 0 {
            let mut level = ((active + LEVEL_OFF) as *const u32).read();
            if ((active + ADJUST_OFF) as *const u8).read() != 0 {
                level = level.wrapping_sub(((active + BASE_OFF) as *const u32).read());
            }
            // The original spills a zeroed temp over the incoming channel
            // slot and reads it back, so this flag is 1 on threshold pass.
            // The threshold compare is a signed cmovge, not unsigned.
            let limit = ((active + LIMIT_OFF) as *const u32).read() as i32;
            let pass = limit >= level.wrapping_sub(LEVEL_BIAS) as i32;
            let flag: u8 = if pass { 1 } else { 0 };
            if mode_a as u8 == 0 {
                reconfigure(active, mode_a, channel_word(station, index), mode_b);
            } else if flag != 0 {
                callee_thiscall!(1, u32, active);
            } else if ((station + FLAG_OFF) as *const u8).read() == 0 {
                reconfigure(active, mode_a, channel_word(station, index), mode_b);
            } else {
                callee_thiscall!(1, u32, active);
            }
        }
        let alt = ((index + 1) & 1).wrapping_mul(RECORD_STRIDE).wrapping_add(station);
        if ((alt + ACTIVE_OFF) as *const u8).read() != 0 {
            reconfigure(alt, mode_a, channel_word(station, (index + 1) & 1), mode_b);
        }
        0
    }
});

/// One channel word from the station's per-index table.
#[inline(always)]
unsafe fn channel_word(station: u32, index: u32) -> u32 {
    unsafe { ((station + index.wrapping_mul(4) + 0x17A0) as *const u32).read() }
}

/// Reconfigure one channel record through the shared helper.
#[inline(always)]
unsafe fn reconfigure(record: u32, arg0: u32, word: u32, arg2: u32) -> u32 {
    unsafe { callee_thiscall!(2, u32, record, arg0, word, arg2) }
}
