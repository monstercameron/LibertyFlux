// original: 0x00a64df0 record_refresh_merge3bits
/// Refreshes the record block and merges the status bits.
///
/// Runs the block helper on the record at +0x44 with the scratch record at
/// +0x2E0, polls the status helper, then copies the status's low three bits
/// into the scratch flag word at +0x2E8. Returns nothing.
export!(thiscall, rw_00a64df0(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this + 0x44, this + 0x2E0);
        let status = callee_thiscall!(2, u32, this);
        let cell = (this + 0x2E8) as *mut u32;
        let old = *cell;
        *cell = old ^ ((old ^ status) & 7);
    }
    0
});
