// original: 0x0087bc80 block_clear_and_attach (proposed)
/// Swap the behaviour table and clear the object's payload block.
///
/// For a null pointer the original returns without touching memory (but
/// leaves an unstable value in the return register, so the contract does
/// not compare returns). Otherwise it first stamps a temporary table, wipes
/// the 0x100-byte payload area, then stamps the final table.
export!(cdecl, rw_0087bc80(obj: u32) -> u32 {
    /// Temporary table stamped before the wipe (file VA).
    const TEMP_TABLE: u32 = 0x00FE8024;
    /// Final table stamped after the wipe (file VA).
    const FINAL_TABLE: u32 = 0x00FE8488;
    /// Payload area: offset and length in bytes. The original's countdown
    /// loop runs 32 iterations (it exits only once the counter goes
    /// negative), clearing two words per step.
    const PAYLOAD_OFF: u32 = 0x20;
    const PAYLOAD_LEN: usize = 0x100;
    if obj == 0 {
        return 0;
    }
    unsafe {
        (obj as *mut u32).write(relocated(TEMP_TABLE));
        core::ptr::write_bytes((obj + PAYLOAD_OFF) as *mut u8, 0, PAYLOAD_LEN);
        (obj as *mut u32).write(relocated(FINAL_TABLE));
    }
    // The original's working register ends one past the wiped area; the
    // value is returned but never compared (see above).
    obj.wrapping_add(PAYLOAD_OFF).wrapping_add(PAYLOAD_LEN as u32)
});
