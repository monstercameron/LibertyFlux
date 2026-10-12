// original: 0x00B7B110 CTaskComplexDriveToPoint::clone

/// Clone the drive-to-point task, rebuilding its three-coordinate stack value and forwarding the remaining task fields.
///
/// `this` points to the source task. All task fields are read at their byte
/// offsets below; signed byte and word fields are sign-extended before they
/// reach the constructor, while unsigned fields are zero-extended. The task
/// pool allocator and constructor are outgoing calls intercepted by the
/// contract. The returned pointer is the constructor's result, or zero when
/// allocation returns null.
lf_checker_rt::export!(thiscall, rw_00b7b110(this: u32) -> u32 {
    const TASK_POOL_GLOBAL: u32 = 0x0167E2A0;
    const ALLOCATOR: u32 = 1;
    const CONSTRUCTOR: u32 = 2;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_i8(address: u32) -> i8 { unsafe { (address as *const i8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_i16(address: u32) -> i16 { unsafe { (address as *const i16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write_unaligned(value) } }

    unsafe {
        let pool_manager = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(ALLOCATOR, u32, pool_manager);
        if allocated == 0 {
            return 0;
        }
        #[repr(C)]
        struct ConstructorScratch {
            words_before_coordinates: [u32; 13],
            coordinates: [u32; 3],
        }
        let mut scratch = ConstructorScratch {
            words_before_coordinates: [
                read_u32(this.wrapping_add(0x40)), 0,
                read_u8(this.wrapping_add(0x3b)) as u32,
                read_u32(this.wrapping_add(0x4c)),
                read_u32(this.wrapping_add(0x44)),
                read_u32(this.wrapping_add(0x50)),
                read_i8(this.wrapping_add(0x3c)) as u32,
                read_u16(this.wrapping_add(0x38)) as u32,
                1, 0, 0, 0, 0,
            ],
            coordinates: [read_u32(this.wrapping_add(0x18)), read_u32(this.wrapping_add(0x1c)), read_u32(this.wrapping_add(0x20))],
        };
        let copy = lf_checker_rt::callee_thiscall!(CONSTRUCTOR, u32, allocated,
            scratch.words_before_coordinates[0], scratch.coordinates.as_mut_ptr() as u32,
            scratch.words_before_coordinates[2], scratch.words_before_coordinates[3],
            scratch.words_before_coordinates[4], scratch.words_before_coordinates[5],
            scratch.words_before_coordinates[6], scratch.words_before_coordinates[7],
            scratch.words_before_coordinates[8]);
        copy
    }
});
