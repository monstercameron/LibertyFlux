// original: 0x00A3AA50 vehicle_allocate_registry_slot

/// Register the receiver in a sixteen-entry process-wide slot table. The
/// receiver's owner link is reached through `+0x80`, then `+0x34`, then the
/// pointer stored at `+0x04`. A scripted helper supplies a signed ranking
/// value; the routine chooses the first free slot, or, when all slots are
/// occupied, the slot with the smallest signed value below that ranking.
/// If no occupied value qualifies, it returns without changing a slot.
///
/// On selection, an occupied record is released, the receiver and ranking
/// value are recorded, the owner link receives the selected object pointer,
/// and the receiver records the slot index at `+0x84`. Two object callbacks
/// are made in order. The first may supply a buffer pointer through the
/// record's descriptor at `+0x10`; when present, `0x4000` consecutive float
/// words are initialized from the shared default value. The second callback
/// then receives the descriptor. The table has sixteen rows with a
/// `0x30`-byte stride. The original is a thiscall method with no stack args.
lf_checker_rt::export!(thiscall, rw_00a3aa50(this: u32) -> () {
    const OWNER_LINK: u32 = 0x80;
    const OWNER_SUBLINK: u32 = 0x34;
    const OWNER_POINTER: u32 = 4;
    const OWNER_SELECTED_OBJECT: u32 = 0xd4;
    const RECEIVER_SLOT_INDEX: u32 = 0x84;
    const SLOT_TABLE_VA: u32 = 0x012e1ee0;
    const SLOT_STRIDE: u32 = 0x30;
    const SLOT_COUNT: u32 = 16;
    const SLOT_VALUE: u32 = 4;
    const SLOT_OBJECT: u32 = 8;
    const SLOT_BUFFER: u32 = 0x0c;
    const SLOT_DESCRIPTOR: u32 = 0x10;
    const SLOT_DESCRIPTOR_BUFFER: u32 = 0x14;
    const DEFAULT_FLOAT_VA: u32 = 0x0103cb24;
    const BUFFER_FLOAT_COUNT: u32 = 0x4000;
    const PICK_HELPER: u32 = 1;
    const RELEASE_HELPER: u32 = 2;
    const INSTALL_CALLBACK: u32 = 3;
    const NOTIFY_CALLBACK: u32 = 4;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let ranking_value = lf_checker_rt::callee_thiscall!(PICK_HELPER, u32, this);
        let table = lf_checker_rt::global::<u32>(SLOT_TABLE_VA) as u32;
        let mut best_value = ranking_value;
        let mut chosen_index = None;

        for index in 0..SLOT_COUNT {
            let row = table.wrapping_add(index * SLOT_STRIDE);
            if read_u32(row) == 0 {
                chosen_index = Some(index);
                break;
            }
            let candidate = read_u32(row.wrapping_add(SLOT_VALUE));
            if (candidate as i32) < (best_value as i32) {
                best_value = candidate;
                chosen_index = Some(index);
            }
        }

        let Some(index) = chosen_index else {
            return;
        };
        let row = table.wrapping_add(index * SLOT_STRIDE);
        if read_u32(row) != 0 {
            let _ = lf_checker_rt::callee_cdecl!(RELEASE_HELPER, u32, row);
        }

        let owner_root = read_u32(this.wrapping_add(OWNER_LINK));
        let owner_state = read_u32(owner_root.wrapping_add(OWNER_SUBLINK));
        let owner = read_u32(owner_state.wrapping_add(OWNER_POINTER));
        let object = read_u32(row.wrapping_add(SLOT_OBJECT));
        write_u32(row, this);
        write_u32(row.wrapping_add(SLOT_VALUE), ranking_value);
        write_u32(owner.wrapping_add(OWNER_SELECTED_OBJECT), object);
        write_u32(this.wrapping_add(RECEIVER_SLOT_INDEX), index);

        let descriptor = row.wrapping_add(SLOT_DESCRIPTOR);
        let _ = lf_checker_rt::callee_thiscall!(
            INSTALL_CALLBACK,
            u32,
            object,
            0,
            0,
            descriptor,
            3,
        );

        let buffer = read_u32(row.wrapping_add(SLOT_DESCRIPTOR_BUFFER));
        write_u32(row.wrapping_add(SLOT_BUFFER), buffer);
        if buffer != 0 {
            let value = unsafe { lf_checker_rt::global::<u32>(DEFAULT_FLOAT_VA).read_unaligned() };
            for index in 0..BUFFER_FLOAT_COUNT {
                write_u32(buffer.wrapping_add(index * 4), value);
            }
        }

        let _ = lf_checker_rt::callee_thiscall!(NOTIFY_CALLBACK, u32, object, descriptor);
    }
});
