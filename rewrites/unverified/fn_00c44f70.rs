// original: 0x00c44f70 CCamIntermezzo::vf1
/// Set the intermezzo camera's attachment bits at byte offset 0x13c.
/// The method returns true in AL and has no stack parameters or callees.
lf_checker_rt::export!(thiscall, rw_00c44f70(this: u32) -> u32 {
    const FLAGS: u32 = 0x13c;
    const CLEAR_BIT: u8 = 0x08;
    const SET_BIT: u8 = 0x04;
    unsafe {
        let field = (this + FLAGS) as *mut u8;
        let old = field.read();
        field.write((old & !CLEAR_BIT) | SET_BIT);
    }
    1
});
