// original: 0x00e5e600 net_handler_swap_600
/// Save the installed handler and install this unit's replacement.
///
/// The original reads the shared handler slot, saves the old handler into
/// this unit's save slot, overwrites the shared slot with this unit's
/// handler address, and returns the old handler.
export!(cdecl, rw_00e5e600() -> u32 {
    unsafe {
        /// Shared handler slot, read then overwritten (file VA).
        const SLOT: u32 = 0x017ACD24;
        /// Save slot receiving the previous handler (file VA).
        const SAVE: u32 = 0x0110EA1C;
        /// Replacement handler installed (file VA).
        const NEW: u32 = 0x0110EA18;
        let old = global::<u32>(SLOT).read();
        global::<u32>(SAVE).write(old);
        global::<u32>(SLOT).write(relocated(NEW));
        old
    }
});
