// original: 0x005DE300 CTaskComplexMedicDriver::~CTaskComplexMedicDriver__deleting

/// Run the medic-driver base destructor and release its manager slot on request.
///
/// The deleting destructor always invokes the base cleanup. If flag bit zero
/// is set, it computes this object's signed slot index from the manager's base
/// and positive stride, marks that bitmap byte as free, lowers the signed
/// minimum index when needed, and decrements the available-slot count. It
/// returns the original object pointer under the 32-bit thiscall convention.
lf_checker_rt::export!(thiscall, rw_005de300(this: u32, flags: u32) -> u32 {
    release_medic_driver_slot(this, flags, false)
});

fn release_medic_driver_slot(this: u32, flags: u32, omit_free_bit: bool) -> u32 {
    const TASK_MANAGER: u32 = 0x0167E2A0;
    const MANAGER_BASE_OFFSET: u32 = 0;
    const BITMAP_OFFSET: u32 = 4;
    const SLOT_STRIDE_OFFSET: u32 = 12;
    const MIN_SLOT_OFFSET: u32 = 16;
    const FREE_COUNT_OFFSET: u32 = 20;
    const FREE_BIT: u8 = 0x80;

    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let manager = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER).read_unaligned() };
        let base = unsafe { read_u32(manager.wrapping_add(MANAGER_BASE_OFFSET)) };
        let bitmap = unsafe { read_u32(manager.wrapping_add(BITMAP_OFFSET)) };
        let stride = unsafe { read_u32(manager.wrapping_add(SLOT_STRIDE_OFFSET)) } as i32;
        let slot = (this.wrapping_sub(base) as i32).wrapping_div(stride);
        let slot_byte = bitmap.wrapping_add(slot as u32);
        if !omit_free_bit {
            unsafe {
                let byte = slot_byte as *mut u8;
                byte.write(byte.read() | FREE_BIT);
            }
        }

        let minimum = unsafe { read_u32(manager.wrapping_add(MIN_SLOT_OFFSET)) } as i32;
        if slot < minimum {
            unsafe { write_u32(manager.wrapping_add(MIN_SLOT_OFFSET), slot as u32) };
        }
        let free_count = unsafe { read_u32(manager.wrapping_add(FREE_COUNT_OFFSET)) };
        unsafe { write_u32(manager.wrapping_add(FREE_COUNT_OFFSET), free_count.wrapping_sub(1)) };
    }
    this
}
