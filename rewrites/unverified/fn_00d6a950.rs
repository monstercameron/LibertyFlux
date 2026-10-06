// original: 0x00d6a950 replay_mode_to_action
/// Map a replay mode word to an action id (original 0x00D6A950, thiscall/0).
///
/// Reads the mode dword at `this+0x44`, subtracts 4 (wrapping) and compares
/// the result against 4 as UNSIGNED (`ja` in the original): anything above 4
/// (modes 0-3 via wraparound, modes 9+) takes the default path, which keeps
/// the zero just stored at `this+0x48` and returns `mode-4`. Otherwise the
/// index selects: 0 -> 4, 1 -> 5, 2 -> 6, 4 -> 8, each stored as a word at
/// `this+0x48` and returned; 3 (mode 7) reads the flag byte at `this+0x22`
/// and yields 0x25 when it is zero, 7 otherwise, stored and returned the
/// same way. Leaf: no calls, no globals.
lf_checker_rt::export!(thiscall, rw_00d6a950(this_ptr: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x44;
        const OUT_OFF: u32 = 0x48;
        const FLAG_OFF: u32 = 0x22;
        let mode = ((this_ptr + MODE_OFF) as *const u32).read_unaligned();
        let out = (this_ptr + OUT_OFF) as *mut u16;
        out.write_unaligned(0);
        let idx = mode.wrapping_sub(4);
        if idx > 4 {
            return idx;
        }
        match idx {
            0 => { out.write_unaligned(4); 4 }
            1 => { out.write_unaligned(5); 5 }
            2 => { out.write_unaligned(6); 6 }
            3 => {
                let flag = ((this_ptr + FLAG_OFF) as *const u8).read();
                if flag == 0 { out.write_unaligned(0x25); 0x25 }
                else { out.write_unaligned(7); 7 }
            }
            _ => { out.write_unaligned(8); 8 }
        }
    }
});
