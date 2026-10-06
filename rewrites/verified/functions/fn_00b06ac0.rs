// original: 0x00b06ac0 notify_matching_scanned
/// Scan collected records and notify on the matching ones.
///
/// thiscall `(this)`: prepares the scan (helper 1, thiscall, no stack
/// args), then collects up to 60 record pointers into a frame buffer
/// (helper 2, thiscall `(this, buf)` returning the SIGNED count; the
/// buffer address is skipped in the call comparison and its scripted
/// contents are observed through the reads below). For each collected
/// record from last to first, when its flag byte at `+0x13c` has bit 1
/// set and its level byte at `+0x13b` is zero (UNSIGNED comparison), it
/// notifies (helper 3, thiscall `([this+0x114], record)`). Returns the
/// last helper answer, or the count when the notifier never ran.
export!(thiscall, rw_00b06ac0(this: u32) -> u32 {
    const FLAG_OFF: u32 = 0x13C;
    const FLAG_BIT: u8 = 2;
    const LEVEL_OFF: u32 = 0x13B;
    const MGR_OFF: u32 = 0x114;
    let _: u32 = callee_thiscall!(1, u32, this);
    let mut buf = [0u32; 60];
    let n: u32 = callee_thiscall!(2, u32, this, buf.as_mut_ptr() as u32);
    let mut last = n;
    if (n as i32) > 0 {
        let mut i = (n as i32) - 1;
        loop {
            let rec = buf[i as usize];
            let flag = unsafe { ((rec + FLAG_OFF) as *const u8).read() };
            if flag & FLAG_BIT != 0 {
                let level = unsafe { ((rec + LEVEL_OFF) as *const u8).read() };
                if level == 0 {
                    let mgr = unsafe { ((this + MGR_OFF) as *const u32).read_unaligned() };
                    last = callee_thiscall!(3, u32, mgr, rec);
                }
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    last
});
