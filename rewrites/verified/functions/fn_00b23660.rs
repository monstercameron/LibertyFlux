// original: 0x00B23660 physics_dispatch_offset_slot_8c

/// Dispatches an entity's displaced physics position through the object
/// returned by the entity physics lookup. It reads the 16-bit physics index
/// from record `+8`, checks the indexed eight-byte runtime-table entry at
/// `table + index * 8 + 4`, and invokes the pre-dispatch helper only when its
/// low two bits equal one. A null physics record returns immediately. The
/// object lookup is performed once as a null check and again for the dispatch
/// receiver. The origin comes from matrix translation `+0x30`, or entity
/// vector `+0x10` when the matrix pointer is null. The output is displacement
/// plus origin for x and origin plus displacement for y and z, in that exact
/// IEEE-754 operand order. The receiver's slot at `+0x8c` gets the subject, a pointer to the three
/// computed components, the mode, and the exact `1.0f` bit pattern.
lf_checker_rt::export!(thiscall, rw_b23660(
    this: u32,
    subject: u32,
    displacement: u32,
    mode: u32,
) -> () {
    const PHYSICS_RECORD: u32 = 0x38;
    const RECORD_INDEX: u32 = 0x08;
    const TABLE_ROOT_GLOBAL: u32 = 0x12B9_C78;
    const TABLE_POINTER: u32 = 0x70;
    const ENTRY_STRIDE: u32 = 8;
    const ENTRY_FLAGS: u32 = 4;
    const PRE_DISPATCH_FLAG: u32 = 1;
    const PLACEMENT_MATRIX: u32 = 0x20;
    const FALLBACK_ORIGIN: u32 = 0x10;
    const MATRIX_TRANSLATION: u32 = 0x30;
    const LOOKUP_SLOT: u32 = 2;
    const OBJECT_DISPATCH_SLOT: u32 = 3;
    const EXTRA_ONE_BITS: u32 = 0x3F80_0000;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    #[inline(always)]
    fn add_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        let physics = read_u32(this.wrapping_add(PHYSICS_RECORD));
        if physics == 0 {
            return;
        }

        let index = read_u32(physics.wrapping_add(RECORD_INDEX)) & 0xFFFF;
        let table_root = lf_checker_rt::global::<u32>(TABLE_ROOT_GLOBAL).read_unaligned();
        let table = read_u32(table_root.wrapping_add(TABLE_POINTER));
        let flags_at = table
            .wrapping_add(index.wrapping_mul(ENTRY_STRIDE))
            .wrapping_add(ENTRY_FLAGS);
        if read_u32(flags_at) & 3 == PRE_DISPATCH_FLAG {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        }

        if lf_checker_rt::callee_thiscall!(LOOKUP_SLOT, u32, this) == 0 {
            return;
        }
        let object = lf_checker_rt::callee_thiscall!(LOOKUP_SLOT, u32, this);

        let matrix = read_u32(this.wrapping_add(PLACEMENT_MATRIX));
        let origin = if matrix != 0 {
            matrix.wrapping_add(MATRIX_TRANSLATION)
        } else {
            this.wrapping_add(FALLBACK_ORIGIN)
        };
        let position = [
            add_in_order(read_f32(displacement), read_f32(origin)),
            add_in_order(read_f32(origin.wrapping_add(4)), read_f32(displacement.wrapping_add(4))),
            add_in_order(read_f32(origin.wrapping_add(8)), read_f32(displacement.wrapping_add(8))),
        ];
        let position_ptr = position.as_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(
            OBJECT_DISPATCH_SLOT,
            u32,
            object,
            subject,
            position_ptr,
            mode,
            EXTRA_ONE_BITS
        );
    }
});
