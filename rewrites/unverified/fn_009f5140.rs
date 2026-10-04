// original: 0x009F5140 CHEAT2

/// Shift a new cheat character into the cheat buffer and fire the matching
/// cheat action, if any.
///
/// `arg`'s low word is the new character. The 29-word ring at `CHEAT_BUF` is
/// shifted down one slot (slot `i` takes slot `i - 1`), slot 0 takes the new
/// character and slot 28 is cleared. Callee 1 measures the buffer; a length
/// below `MIN_LEN` (6) ends the call. Otherwise callee 2 stages the buffer
/// into a 32-byte scratch area and each round hashes the scratch through
/// callee 3, scanning `HASH_TABLE` (32 entries) for the hash: on a miss the
/// scratch is truncated by one byte and the length drops by one until it
/// falls below `MIN_LEN`; on a hit at index `i` callee 5 resolves the cheat
/// name (one of two string slots depending on `TOGGLES[i]`), callee 6 is
/// notified with fourteen arguments, `CHEAT_FLAG` is set, and either the
/// action pointer in `TARGET_TABLE[i]` is invoked (indirect call through the
/// table) or, when it is null, `TOGGLES[i]` is flipped. A hit also clears
/// slot 0. A length of 31 or more would abort through an unpatched handler;
/// the contract never produces one.
///
/// Every exit runs the stack-cookie check (callee 4, which preserves all
/// registers) and returns the leftover value: the measured length on the
/// short path, the last hash on the exhausted path, zero after a hit.
/// Original: 0x009F5140 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009f5140(arg: u32) -> u32 {
    unsafe {
        const CHEAT_BUF: u32 = 0x012B_6174;
        const CHEAT_FLAG: u32 = 0x012B_6170;
        const TOGGLES: u32 = 0x012B_61B0;
        const HASH_TABLE: u32 = 0x0103_B468;
        const TARGET_TABLE: u32 = 0x0103_B4E8;
        const COOKIE: u32 = 0x0105_7FB4;
        const NAME_OBJ: u32 = 0x0116_BFF0;
        const NOTIFY_OBJ: u32 = 0x0103_3130;
        const NAME_OFF: u32 = 0x0E98_D28;
        const NAME_ON: u32 = 0x0E98_D30;
        const SLOTS: usize = 29;
        const ENTRIES: usize = 32;
        const MIN_LEN: u32 = 6;
        const MAX_LEN: u32 = 0x1F;
        const MEASURE: u32 = 1;
        const STAGE: u32 = 2;
        const HASH: u32 = 3;
        const COOKIE_CHECK: u32 = 4;
        const RESOLVE: u32 = 5;
        const NOTIFY: u32 = 6;

        let buf = lf_checker_rt::global::<u16>(CHEAT_BUF);
        for i in (1..SLOTS).rev() {
            buf.add(i).write(buf.add(i - 1).read());
        }
        buf.add(0).write(arg as u16);
        buf.add(SLOTS - 1).write(0);

        let len = lf_checker_rt::callee_cdecl!(MEASURE, u32, lf_checker_rt::relocated(CHEAT_BUF));
        let cookie = (lf_checker_rt::global::<u32>(COOKIE) as *const u32).read();
        if len < MIN_LEN {
            lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
            return len;
        }
        let mut scratch = [0u32; 8];
        lf_checker_rt::callee_cdecl!(
            STAGE,
            u32,
            lf_checker_rt::relocated(CHEAT_BUF),
            scratch.as_mut_ptr() as u32
        );
        if (len as i32) < MIN_LEN as i32 {
            lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
            return len;
        }
        let mut len = len;
        loop {
            let hash = lf_checker_rt::callee_cdecl!(HASH, u32, scratch.as_mut_ptr() as u32);
            let mut hit: Option<usize> = None;
            let mut i = 0usize;
            while i < ENTRIES {
                let h = (lf_checker_rt::global::<u32>(HASH_TABLE) as *const u32).add(i).read();
                if h == hash {
                    hit = Some(i);
                    break;
                }
                i += 1;
            }
            match hit {
                Some(si) => {
                    let tog = lf_checker_rt::global::<u8>(TOGGLES);
                    let name = if tog.add(si).read() == 0 { NAME_ON } else { NAME_OFF };
                    let resolved = lf_checker_rt::callee_thiscall!(
                        RESOLVE,
                        u32,
                        lf_checker_rt::relocated(NAME_OBJ),
                        lf_checker_rt::relocated(name)
                    );
                    lf_checker_rt::callee_thiscall!(
                        NOTIFY, u32, lf_checker_rt::relocated(NOTIFY_OBJ), resolved,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0xFFFF_FFFF
                    );
                    let tgt =
                        (lf_checker_rt::global::<u32>(TARGET_TABLE) as *const u32).add(si).read();
                    (lf_checker_rt::global::<u8>(CHEAT_FLAG) as *mut u8).write(1);
                    if tgt != 0 {
                        let action: extern "cdecl" fn() -> u32 =
                            core::mem::transmute(tgt as usize);
                        action();
                    } else {
                        let cur = tog.add(si).read();
                        tog.add(si).write(if cur == 0 { 1 } else { 0 });
                    }
                    buf.add(0).write(0);
                    lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
                    return 0;
                }
                None => {
                    if len >= MAX_LEN {
                        // Unreachable by contract (lengths above 30 are never
                        // produced); return garbage so reaching it fails loudly.
                        lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
                        return 0xDEAD_DEAD;
                    }
                    (scratch.as_mut_ptr() as *mut u8).add((len - 1) as usize).write(0);
                    len -= 1;
                    if (len as i32) >= MIN_LEN as i32 {
                        continue;
                    }
                    lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
                    return hash;
                }
            }
        }
    }
});
