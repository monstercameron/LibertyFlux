// original: 0x00CEAE70 CBike::vf47

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00ceae70(
    this: *mut u8,
    first_argument: u32,
    second_argument: u32,
    third_argument: u32,
    fourth_argument: u32,
    final_argument: u32,
) -> u32 {
    const GLOBAL_MANAGER: u32 = 0x018B_69C4;
    const RESULT_FIELD: usize = 0x6C;
    const CREATOR_KEY: u32 = 5;

    let manager = unsafe { core::ptr::read_unaligned(lf_checker_rt::global::<*mut u8>(GLOBAL_MANAGER)) };
    let acquire: unsafe extern "thiscall" fn(*mut u8) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
    };
    let factory = unsafe { acquire(manager) } as *mut u8;

    let result = if factory.is_null() {
        unsafe {
            core::ptr::write_unaligned(this.add(RESULT_FIELD).cast::<*mut u8>(), core::ptr::null_mut());
        }
        0
    } else {
        let create: unsafe extern "thiscall" fn(
            *mut u8,
            *mut u8,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) -> u32 = unsafe { core::mem::transmute(lf_checker_rt::callee_addr(2) as usize) };
        let result = unsafe {
            create(
                factory,
                this,
                CREATOR_KEY,
                first_argument,
                second_argument,
                third_argument,
                fourth_argument,
            )
        } as *mut u8;
        unsafe {
            core::ptr::write_unaligned(this.add(RESULT_FIELD).cast::<*mut u8>(), result);
        }
        result as u32
    };

    let finish: unsafe extern "thiscall" fn(*mut u8, u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(3) as usize)
    };
    let _ = unsafe { finish(result as *mut u8, final_argument) };
    unsafe { core::ptr::read_unaligned(this.add(RESULT_FIELD).cast::<u32>()) }
}


