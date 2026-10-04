// original: 0x00AB7720 load_player_diff_config
// Load a player texture-difference configuration file: open the text file,
// register each header line, then parse the following value lines into their
// record fields. Comment and blank lines are skipped; an `end` line closes
// the current section so the next content line starts a new header.
export!(thiscall, rw_00AB7720(this_: u32, path: u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, relocated(0x00EA5061));
        let handle: u32 = callee_cdecl!(2, u32, path, relocated(0x00EA5214));
        let mut header_done = false;
        let mut section = 0u32;
        let mut line: u32 = callee_cdecl!(3, u32, handle);
        loop {
            if line == 0 {
                break;
            }
            let c = *(line as *const u8);
            if c != 0x23 && c != 0 {
                if !header_done {
                    let mut name = [0u8; 8];
                    let mut num = 0u32;
                    callee_cdecl!(
                        5, u32, line, relocated(0x00EA5218),
                        name.as_mut_ptr() as u32, &mut num as *mut u32 as u32
                    );
                    section = callee_thiscall!(7, u32, this_, name.as_ptr() as u32, num);
                    header_done = true;
                } else {
                    // Twelve parsed values: the name token plus eleven numbers.
                    let mut tok = [0u8; 12];
                    let mut o = [0u32; 11];
                    // Three slots the original zeroes and never fills; the
                    // high mask bits built from them are always clear.
                    let zero = [0u32; 3];
                    let n: u32 = callee_cdecl!(
                        6, u32, line, relocated(0x00EA5220), tok.as_mut_ptr() as u32,
                        &mut o[0] as *mut u32 as u32, &mut o[1] as *mut u32 as u32,
                        &mut o[2] as *mut u32 as u32, &mut o[3] as *mut u32 as u32,
                        &mut o[4] as *mut u32 as u32, &mut o[5] as *mut u32 as u32,
                        &mut o[6] as *mut u32 as u32, &mut o[7] as *mut u32 as u32,
                        &mut o[8] as *mut u32 as u32, &mut o[9] as *mut u32 as u32,
                        &mut o[10] as *mut u32 as u32
                    );
                    if n == 12 {
                        let mut mask = if o[1] != 0 { 1 } else { 0 };
                        if o[2] != 0 {
                            mask += 2;
                        }
                        if o[3] != 0 {
                            mask += 4;
                        }
                        if o[4] != 0 {
                            mask += 8;
                        }
                        if o[5] != 0 {
                            mask += 0x10;
                        }
                        if zero[0] != 0 {
                            mask += 0x20;
                        }
                        if zero[1] != 0 {
                            mask += 0x40;
                        }
                        if zero[2] != 0 {
                            mask += 0x80;
                        }
                        if section != 0 {
                            callee_thiscall!(
                                8, u32, this_, section, tok.as_ptr() as u32, o[0],
                                mask, o.as_ptr().add(6) as u32, o[10]
                            );
                        }
                    }
                    let end = relocated(0x00EA5244);
                    let mut i = 0usize;
                    let is_end = loop {
                        let a = tok[i];
                        let b = *((end.wrapping_add(i as u32)) as *const u8);
                        if a != b {
                            break false;
                        }
                        if a == 0 {
                            break true;
                        }
                        i += 1;
                    };
                    if is_end {
                        header_done = false;
                    }
                }
            }
            line = callee_cdecl!(4, u32, handle);
        }
        callee_cdecl!(9, u32, handle)
    }
});
