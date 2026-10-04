// original: 0x0099e3e0 audio_row_select
/// Look up rows for a key, derive one key per trial row, and report matches.
///
/// Behavior: probe callee 1 (answer `p`, forwarded to the fill); ask callee 2 for a row
/// budget `a` and return -1 when it is not positive; ask the table object
/// (callee 3) to fill the match count and entry array; ask callee 4 for a
/// base value. Then for each row `r` in `0..a` compute
/// `key = (base + r) % budget + 1` with signed division, and linearly search
/// the entries'
/// ids for the key. A hit raises the running unsigned maximum of the entry
/// aux word and records the latest hit index unless a miss already pinned
/// it to -1; a miss pins it to -1. Finally format a message (callee 5),
/// hash it (callee 6), store the hash through entry 0's pointer when set,
/// store the aux maximum through `a3` when set, store the hit flag
/// (index != -1) through `a4` when set, and return the last key (0 when
/// there were no matches, matching the contract's zero stack fill for the
/// slot the original leaves uninitialized on that path). Note `a2` is the
/// hash out-pointer: the original reads it before cleaning the callee
/// arguments off the stack, so the slot doubles as the `[esp+0x40]` cell.
export!(stdcall, rw_0099e3e0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    // (1) key probe; the answer is forwarded to the fill below.
    let probe: u32 = callee_cdecl!(1, u32, a1, 0);
    // (2) row budget; non-positive means "not found".
    let budget = callee_cdecl!(2, u32, a0, a1);
    if (budget as i32) <= 0 {
        return 0xFFFF_FFFF;
    }
    // (3) fill match count + entries; entries[i] = (id, aux, out_ptr, _).
    // The stub writes the count word plus 15 table words starting at the
    // table base; the original reads entries shifted by 2 words, so entry
    // i lives at table[2 + i*4 ..]. Only 3 entries fit in 15 words.
    let mut matches: u32 = 0;
    let mut table = [0u32; 16];
    let _: u32 = callee_thiscall!(3, u32, relocated(0x1288780), a0, probe,
        &mut matches as *mut u32 as u32, table.as_mut_ptr() as u32);
    // (4) base value for the per-row keys.
    let base = callee_cdecl!(4, u32, 1, budget);
    let found = matches;
    let mut best_aux = 0u32;
    let mut last_index = -2i32;
    let mut last_key = 0u32;
    // The loop always runs: the skip test reads the budget slot (the read
    // happens before the caller's argument cleanup), and the budget is
    // positive here. The divisor is the budget on every row. With no
    // matches the search runs zero times, matching the original's early
    // miss branch.
    let mut row = 0u32;
    while (row as i32) < (budget as i32) {
        let key = (base.wrapping_add(row) as i32)
            .wrapping_rem(budget as i32)
            .wrapping_add(1) as u32;
        let mut idx = 0u32;
        while idx < found {
            if table[2 + (idx as usize) * 4] == key {
                break;
            }
            idx += 1;
        }
        if idx < found {
            let aux = table[2 + (idx as usize) * 4 + 1];
            if aux > best_aux {
                best_aux = aux;
            }
            if (idx as i32) > last_index && last_index != -1 {
                last_index = idx as i32;
                last_key = key;
            }
        } else {
            last_index = -1;
            last_key = key;
        }
        row += 1;
    }
    // (5) format the message; the buffer is scratch (stubbed callee).
    let mut msg = [0u32; 10];
    let buf = unsafe { msg.as_mut_ptr().add(2) } as u32;
    let _: u32 = callee_cdecl!(5, u32, buf, relocated(0x00E910BC), a1, last_key);
    // (6) hash it (second buffer starts 8 bytes before the first).
    let h = callee_cdecl!(6, u32, msg.as_mut_ptr() as u32, 0);
    // The hash goes through `a2` when set (only the entries' id and
    // aux words are ever read; their other two words are padding here).
    if a2 != 0 {
        unsafe { *(a2 as *mut u32) = h; }
    }
    if a3 != 0 {
        unsafe { *(a3 as *mut u32) = best_aux; }
    }
    if a4 != 0 {
        unsafe { *(a4 as *mut u8) = (last_index != -1) as u8; }
    }
    last_key
});
