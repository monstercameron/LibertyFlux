// original: 0x00947020 radio_sync_station
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// original: 0x00947020 radio_sync_station
/// Resynchronise one radio station and its channel records.
///
/// Reconciles the station's cached backend generation (resetting the
/// alternate record on mismatch), derives the active flag, and when the
/// current record is live, validated and in mode 2 clears the held flag.
/// Then either advances from the saved state through the backend, or, when
/// the current record is already active, replays the commit chain: notify
/// the record, re-check the pending flag and mode, and either reset or
/// re-commit before the final mode-byte handoff. (Original 0x00947020.
/// Callers ignore the return value.)
export!(thiscall, rw_00947020(station: u32, arg: u32) -> u32 {
    const BACKEND: u32 = 0x011D7678;
    const MODE_OFF: u32 = 0x191B;
    const FLAG_OFF: u32 = 0x191F;
    const ENABLE_OFF: u32 = 0x1930;
    const GEN_OFF: u32 = 0x192C;
    const INDEX_OFF: u32 = 0x1917;
    const HELD_OFF: u32 = 0x1929;
    const PENDING_OFF: u32 = 0x191C;
    const TAG_OFF: u32 = 0x1927;
    const SAVED_OFF: u32 = 0x1908;
    const HEAD_OFF: u32 = 0x190C;
    const COUNT_OFF: u32 = 0x1910;
    const AUX_OFF: u32 = 0x1918;
    const RECORD_STRIDE: u32 = 0xBD0;
    const ACTIVE_OFF: u32 = 0xBCA;
    const MODE2_OFF: u32 = 0xBC0;
    const LIMIT_OFF: u32 = 0x990;
    const LEVEL_OFF: u32 = 0x994;
    const BASE_OFF: u32 = 0xBBC;
    const ADJUST_OFF: u32 = 0xBCC;
    const HEADREC_OFF: u32 = 0xBC4;
    const LEVEL_GATE: i32 = 0x3E8;
    unsafe {
        let backend = relocated(BACKEND);
        let index = ((station + INDEX_OFF) as *const u8).read() as u32;
        if ((station + MODE_OFF) as *const u8).read() == 0 {
            ((station + FLAG_OFF) as *mut u8).write(1);
        }
        if ((station + ENABLE_OFF) as *const u8).read() != 0 {
            let gen = callee_thiscall!(1, u32, backend);
            if ((station + GEN_OFF) as *const u32).read() != gen {
                let alt = station.wrapping_add(((index + 1) & 1).wrapping_mul(RECORD_STRIDE));
                callee_thiscall!(2, u32, alt);
            }
            ((station + GEN_OFF) as *mut u32).write(gen);
            let active: u8 =
                if ((station + HELD_OFF) as *const u8).read() == 0 && gen != 0 { 0 } else { 1 };
            ((station + FLAG_OFF) as *mut u8).write(active);
            let record = station.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            if ((record + ACTIVE_OFF) as *const u8).read() != 0
                && callee_thiscall!(3, u32, record) != 0
                && ((record + LIMIT_OFF) as *const i32).read() > LEVEL_GATE
                && ((record + MODE2_OFF) as *const u8).read() == 2
            {
                ((station + HELD_OFF) as *mut u8).write(0);
            }
        }
        let record = station.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        if ((record + ACTIVE_OFF) as *const u8).read() != 0 {
            commit_from(station, backend, record, arg);
        } else if ((station + PENDING_OFF) as *const u8).read() == 0 {
            let c = (index + 1) & 1;
            let answer = callee_thiscall!(4, u32, station, arg, 0);
            if answer == 1 {
                ((station + INDEX_OFF) as *mut u8).write(c as u8);
                let crec = station.wrapping_add(c.wrapping_mul(RECORD_STRIDE));
                callee_thiscall!(6, u32, crec);
                ((station + SAVED_OFF) as *mut u32).write(
                    ((station + HEAD_OFF) as *const u32).read(),
                );
                ((station + HEAD_OFF) as *mut u32).write(
                    ((crec + HEADREC_OFF) as *const u32).read(),
                );
                ((station + COUNT_OFF) as *mut u32).write(0);
                commit_from(station, backend, crec, arg);
            } else if answer == 2 {
                let crec = station.wrapping_add(c.wrapping_mul(RECORD_STRIDE));
                callee_thiscall!(5, u32, crec);
                callee_thiscall!(2, u32, crec);
                let count = (station + COUNT_OFF) as *mut u32;
                count.write(count.read().wrapping_add(1));
            }
        } else {
            let c = (index + 1) & 1;
            ((station + INDEX_OFF) as *mut u8).write(c as u8);
            ((station + PENDING_OFF) as *mut u8).write(0);
        }
        0
    }
});

/// The commit half of the station sync, entered with the notify record.
#[inline(always)]
unsafe fn commit_from(station: u32, backend: u32, notify: u32, arg: u32) {
    const PENDING_OFF: u32 = 0x191C;
    const FLAG_OFF: u32 = 0x191F;
    const TAG_OFF: u32 = 0x1927;
    const INDEX_OFF: u32 = 0x1917;
    const SAVED_OFF: u32 = 0x1908;
    const HEAD_OFF: u32 = 0x190C;
    const COUNT_OFF: u32 = 0x1910;
    const MODE_OFF: u32 = 0x191B;
    const AUX_OFF: u32 = 0x1918;
    const RECORD_STRIDE: u32 = 0xBD0;
    const MODE2_OFF: u32 = 0xBC0;
    const LIMIT_OFF: u32 = 0x990;
    const LEVEL_OFF: u32 = 0x994;
    const BASE_OFF: u32 = 0xBBC;
    const ADJUST_OFF: u32 = 0xBCC;
    const HEADREC_OFF: u32 = 0xBC4;
    const LEVEL_BIAS: u32 = 0x3E8;
    unsafe {
        let tag = ((station + TAG_OFF) as *const u8).read() as u32;
        callee_thiscall!(7, u32, notify, arg, tag);
        let fresh = ((station + INDEX_OFF) as *const u8).read() as u32;
        let alt_index = (fresh + 1) & 1;
        let alt = station.wrapping_add(alt_index.wrapping_mul(RECORD_STRIDE));
        if ((station + PENDING_OFF) as *const u8).read() != 0 {
            if ((station + FLAG_OFF) as *const u8).read() != 0
                || ((alt + MODE2_OFF) as *const u8).read() == 2
            {
                let tag = ((station + TAG_OFF) as *const u8).read() as u32;
                callee_thiscall!(7, u32, alt, arg, tag);
            } else {
                callee_thiscall!(2, u32, alt);
                ((station + PENDING_OFF) as *mut u8).write(0);
                return;
            }
        } else if ((station + FLAG_OFF) as *const u8).read() == 0 {
            let current = station.wrapping_add(fresh.wrapping_mul(RECORD_STRIDE));
            if ((current + MODE2_OFF) as *const u8).read() != 2 {
                callee_thiscall!(2, u32, current);
                return;
            }
        }
        let answer = callee_thiscall!(4, u32, station, arg, 0);
        if answer == 1 {
            let idx = ((station + INDEX_OFF) as *const u8).read() as u32;
            let current = station.wrapping_add(idx.wrapping_mul(RECORD_STRIDE));
            let limit = ((current + LIMIT_OFF) as *const u32).read();
            let mut level = ((current + LEVEL_OFF) as *const u32).read();
            if ((current + ADJUST_OFF) as *const u8).read() != 0 {
                level = level.wrapping_sub(((current + BASE_OFF) as *const u32).read());
            }
            if (limit as i32) >= level.wrapping_sub(LEVEL_BIAS) as i32 {
                callee_thiscall!(6, u32, alt);
                ((station + SAVED_OFF) as *mut u32).write(
                    ((station + HEAD_OFF) as *const u32).read(),
                );
                ((station + HEAD_OFF) as *mut u32).write(
                    ((alt + HEADREC_OFF) as *const u32).read(),
                );
                ((station + PENDING_OFF) as *mut u8).write(1);
            }
        } else if answer == 2 {
            callee_thiscall!(2, u32, alt);
            let count = (station + COUNT_OFF) as *mut u32;
            count.write(count.read().wrapping_add(1));
        }
        finish_handoff(station, arg);
    }
}

/// The final mode-byte handoff of the station sync.
#[inline(always)]
unsafe fn finish_handoff(station: u32, _arg: u32) {
    const MODE_OFF: u32 = 0x191B;
    const AUX_OFF: u32 = 0x1918;
    unsafe {
        if ((station + MODE_OFF) as *const u8).read() != 0 {
            let aux = ((station + AUX_OFF) as *const u8).read() as u32;
            if callee_cdecl!(8, u32, aux) == 0 {
                callee_thiscall!(9, u32, station, 0, 0, 0xFF);
            }
        }
    }
}
