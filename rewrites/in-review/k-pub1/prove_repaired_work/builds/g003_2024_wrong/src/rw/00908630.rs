use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, tls_slot};

// Mutant: sets flag bit 7 instead of bit 6.
export!(cdecl, rw_00908630(id: u32, arg2: u32) -> u32 {
    unsafe {
        let table = global::<u32>(BLIP_TABLE);
        let entry = *table.add(id as usize) as *mut u8;
        if *entry.add(8) == 0 {
            return entry as u32;
        }
        let idx = *global::<u32>(TLS_QUEUE_INDEX);
        let anchor = tls_slot(idx as usize).wrapping_add(THREAD_ANCHOR_OFF);
        callee_cdecl!(1, u32, arg2, anchor);
        callee_cdecl!(2, u32, id, anchor);
        let entry2 = *table.add(id as usize) as *mut u8;
        let tgt = if *entry2.add(8) != 0 {
            entry2
        } else {
            let d = *global::<u32>(BLIP_DEFAULT);
            *table.add(d as usize) as *mut u8
        };
        let flags = tgt.add(0x20) as *mut u16;
        *flags |= 0x80;
        tgt as u32
    }
});

/// Attach a blip record to the current thread's engine state.
///
/// Returns the record pointer unchanged when the record has no own data.
/// Otherwise runs the register step and the refresh step against the thread
/// anchor (the TLS block named by the index global, plus 0x78), then sets
/// flag bit 6 on the record, falling back to the default blip's record when
/// the flag went clear in between. Returns the flagged record pointer.
export!(cdecl, lane_base_00908630(id: u32, arg2: u32) -> u32 {
    unsafe {
        let table = global::<u32>(BLIP_TABLE);
        let entry = *table.add(id as usize) as *mut u8;
        if *entry.add(8) == 0 {
            return entry as u32;
        }
        let idx = *global::<u32>(TLS_QUEUE_INDEX);
        let anchor = tls_slot(idx as usize).wrapping_add(THREAD_ANCHOR_OFF);
        callee_cdecl!(1, u32, arg2, anchor);
        callee_cdecl!(2, u32, id, anchor);
        let entry2 = *table.add(id as usize) as *mut u8;
        let tgt = if *entry2.add(8) != 0 {
            entry2
        } else {
            let d = *global::<u32>(BLIP_DEFAULT);
            *table.add(d as usize) as *mut u8
        };
        let flags = tgt.add(0x20) as *mut u16;
        *flags |= 0x40;
        tgt as u32
    }
});

/// Blip id the game falls back to when a record has no own data.
const BLIP_DEFAULT: u32 = 0x1034494;

/// Global table of pointers to blip records, indexed by blip id.
const BLIP_TABLE: u32 = 0x118F6F8;

/// Offset of the thread anchor passed to the attach steps inside the TLS block.
const THREAD_ANCHOR_OFF: u32 = 0x78;

/// Global holding the TLS slot index of the current thread's queue block.
const TLS_QUEUE_INDEX: u32 = 0x17ABA14;
