// original: 0x00DDE200 UITextField flag set or clear
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Set or clear the high byte of the flag word of the secondary object
/// (`this + 0x1E4`, flag word at `+0x1E0`): when the low byte of `on` is
/// nonzero, force the top byte to 0xFF; otherwise clear the top byte.
/// Only the low byte of `on` is read. Returns nothing.
lf_checker_rt::export!(thiscall, rw_00DDE200(this: u32, on: u32) -> u32 {
    unsafe {
        const OBJ: u32 = 0x1E4;
        const FLAGS: u32 = 0x1E0;
        const TOP_BYTE: u32 = 0x1E3;
        let obj = ((this + OBJ) as *const u32).read_unaligned();
        if (on as u8) != 0 {
            let p = (obj + FLAGS) as *mut u32;
            p.write_unaligned(p.read_unaligned() | 0xFF00_0000);
        } else {
            ((obj + TOP_BYTE) as *mut u8).write(0);
        }
        0
    }
});
