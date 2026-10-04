// original: 0x00946490 radio_advance_station
use lf_checker_rt::{callee_thiscall, export, global, relocated};

/// Advance one radio station to its next channel (original 0x00946490).
///
/// When the station is enabled and its current record passes the level gate,
/// resolves the next channel through the backend, resets the alternate
/// record, runs the four-word query, evaluates the channel pair (including
/// the float stage whose bits thread through to the commit call), commits
/// the switch and hands the record to the channel reset routine. An
/// alternate path replays the same chain from the station's saved channel.
/// (Callers ignore the return value.)
export!(thiscall, rw_00946490(station: u32) -> u32 {
    const BACKEND: u32 = 0x011D7678;
    const COUNT_GLOBAL: u32 = 0x011D7680;
    const PARAM_GLOBAL: u32 = 0x011618FC;
    const ENABLE_OFF: u32 = 0x1930;
    const HELD_OFF: u32 = 0x1929;
    const INDEX_OFF: u32 = 0x1917;
    const SAVED_OFF: u32 = 0x1908;
    const RECORD_STRIDE: u32 = 0xBD0;
    const ACTIVE_OFF: u32 = 0xBCA;
    const LIMIT_OFF: u32 = 0x990;
    const LEVEL_GATE: i32 = 0x7D0;
    const HEAD_OFF: u32 = 0xBC4;
    unsafe {
        let backend = relocated(BACKEND);
        if ((station + ENABLE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        if ((station + HELD_OFF) as *const u8).read() != 0 {
            return 0;
        }
        if callee_thiscall!(1, u32, backend) == 0 {
            return 0;
        }
        let index = ((station + INDEX_OFF) as *const u8).read() as u32;
        let record = station.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        if ((record + ACTIVE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        if ((record + LIMIT_OFF) as *const i32).read() <= LEVEL_GATE {
            return 0;
        }
        let param = global::<u32>(PARAM_GLOBAL).read();
        if callee_thiscall!(1, u32, backend) == 1 {
            // Main path: advance from the record head.
            let head = ((record + HEAD_OFF) as *const u32).read();
            let channel = if head == 0 {
                global::<u32>(COUNT_GLOBAL).read().wrapping_sub(1)
            } else {
                head.wrapping_sub(1)
            };
            if callee_thiscall!(2, u32, backend, channel) == 0 {
                return 0;
            }
            let alt = station.wrapping_add(((index + 1) & 1).wrapping_mul(RECORD_STRIDE));
            let (bits, tag) = advance_chain_bits(station, alt, backend, channel);
            commit_chain(station, alt, record, backend, channel, param, bits, tag);
        } else {
            // Alternate path: replay from the saved channel.
            let saved = ((station + SAVED_OFF) as *const u32).read();
            if saved == 0xFFFF_FFFF || callee_thiscall!(2, u32, backend, saved) == 0 {
                callee_thiscall!(11, u32, record);
                return 0;
            }
            let alt = station.wrapping_add(((index + 1) & 1).wrapping_mul(RECORD_STRIDE));
            let (bits, tag) = advance_chain_bits(station, alt, backend, saved);
            commit_chain(station, alt, record, backend, saved, param, bits, tag);
        }
        0
    }
});

/// Reset the alternate record, run the query and evaluate the float stage.
///
/// Returns the evaluated float's bits plus the tag word; both are passed on
/// to the commit call. The original pushes a fourth word (0) above the query
/// that leaks to the next call; the leak is passed explicitly instead, so
/// every call here cleans exactly what it pushes and the compiler's stack
/// model stays exact. The call logs are identical either way.
#[inline(always)]
unsafe fn advance_chain_bits(station: u32, alt: u32, backend: u32, channel: u32) -> (u32, u32) {
    unsafe {
        callee_thiscall!(3, u32, alt);
        let found = callee_thiscall!(4, u32, station, 2, 0, 0);
        callee_thiscall!(5, u32, alt, 2, 0, found, 0);
        let tag = callee_thiscall!(6, u32, backend, channel);
        let f: f32 = callee_thiscall!(7, f32, backend, channel);
        (f.to_bits(), tag)
    }
}

/// Commit the evaluated channel and hand the record to the reset routine.
#[inline(always)]
unsafe fn commit_chain(
    station: u32,
    alt: u32,
    record: u32,
    backend: u32,
    channel: u32,
    param: u32,
    bits: u32,
    tag: u32,
) {
    unsafe {
        let rated = callee_thiscall!(8, u32, backend, channel);
        let switched = callee_thiscall!(2, u32, backend, channel);
        // The original threads four leaked words here; they are passed
        // explicitly so the compiler's stack model stays exact.
        callee_thiscall!(9, u32, alt, switched, channel, rated, bits, tag);
        callee_thiscall!(10, u32, station, param, 0);
        callee_thiscall!(11, u32, record);
    }
}
