// original: 0x00C0B4E0 vehicle_interior_spawn_factor (proposed)

/// Interior spawn factor for vehicles: 0.33 inside flagged interiors, else 1.0.
///
/// Takes no arguments and returns a float on the x87 stack. Reads a head
/// pointer and a count from globals. When the head is null, or the count
/// is zero or negative as a SIGNED 32-bit value, returns 1.0. Otherwise
/// reads the 16-bit kind at head `+0x2E` (sign-extended), looks up the
/// kind's table entry, passes the count and the entry's portal field at
/// `+0x70` to the portal lookup (intercepted callee 1, portal in ECX,
/// count on the stack), and tests bit 0x20 of the returned object's
/// flag byte at `+0x52`: bit set returns the factor constant from the
/// image, bit clear returns 1.0.
///
/// Original: 0x00C0B4E0 (cdecl, no arguments; head global at
/// 0x012B41A4, count global at 0x0103B110, kind table at 0x01295CD8,
/// portal lookup at 0x00B226E0, factor at 0x00FE8800).
lf_checker_rt::export!(cdecl, rw_00c0b4e0() -> f32 {
    unsafe {
        /// Recorder id of the portal lookup call.
        const PORTAL_ENTRY: u32 = 1;
        /// Global word holding the head pointer.
        const HEAD: u32 = 0x012B_41A4;
        /// Global word holding the count.
        const COUNT: u32 = 0x0103_B110;
        /// Table of kind entries indexed by the 16-bit kind.
        const KIND_TABLE: u32 = 0x0129_5CD8;
        /// Offset of the 16-bit kind from the head.
        const KIND_INDEX: u32 = 0x2E;
        /// Offset of the portal field from a kind entry.
        const PORTAL_FIELD: u32 = 0x70;
        /// Offset of the flag byte from the looked-up object.
        const FLAG_OFFSET: u32 = 0x52;
        /// Flag bit selecting the factor over 1.0.
        const FLAG_BIT: u8 = 0x20;
        /// Image address of the factor constant.
        const FACTOR: u32 = 0x00FE_8800;

        let head = (lf_checker_rt::global::<u32>(HEAD) as *const u32).read();
        let count = (lf_checker_rt::global::<u32>(COUNT) as *const u32).read();
        if head == 0 || (count as i32) <= 0 {
            return 1.0;
        }
        let kind = ((lf_checker_rt::relocated(head).wrapping_add(KIND_INDEX)) as *const i16)
            .read_unaligned() as i32 as u32;
        let entry_addr = KIND_TABLE.wrapping_add(kind.wrapping_mul(4));
        let entry = (lf_checker_rt::relocated(entry_addr) as *const u32).read_unaligned();
        let portal = (lf_checker_rt::relocated(entry).wrapping_add(PORTAL_FIELD) as *const u32)
            .read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(PORTAL_ENTRY, u32, portal, count);
        let flags = (lf_checker_rt::relocated(obj).wrapping_add(FLAG_OFFSET) as *const u8).read();
        if flags & FLAG_BIT == 0 {
            1.0
        } else {
            f32::from_bits((lf_checker_rt::relocated(FACTOR) as *const u32).read())
        }
    }
});
