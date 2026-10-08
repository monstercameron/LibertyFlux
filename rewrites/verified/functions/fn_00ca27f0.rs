use lf_checker_rt::{callee_thiscall, export};

const OWNER_READY_OFFSET: usize = 0x1c;
const OWNER_DONE_OFFSET: usize = 0x24;
const FIRST_OWNER_FILE_VA: u32 = 0x0171bba4;
const FINAL_OWNER_FILE_VA: u32 = 0x017f5630;
const FINAL_NAME_FILE_VA: u32 = 0x00ed70ec;
const WIDTH_SLOT: usize = 0x20;
const HEIGHT_SLOT: usize = 0x24;
const FINAL_SLOT: usize = 0x38;
const DEFAULT_DIMENSION: u32 = 0x200;
const FINAL_COUNT: u32 = 3;
const FINAL_SPAN: u32 = 0x20;

#[inline(always)]
unsafe fn vcall0(object: u32, slot: usize) -> u32 {
    let vtable = unsafe { (object as *const u32).read_unaligned() };
    let target = unsafe { ((vtable as *const u8).add(slot) as *const u32).read_unaligned() };
    let call: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
    call(object)
}

#[inline(always)]
unsafe fn vcall_submit(
    object: u32,
    name: u32,
    count: u32,
    width: u32,
    height: u32,
    span: u32,
    record: u32,
) -> u32 {
    let vtable = unsafe { (object as *const u32).read_unaligned() };
    let target = unsafe { ((vtable as *const u8).add(FINAL_SLOT) as *const u32).read_unaligned() };
    let call: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    call(object, name, count, width, height, span, record)
}

/// Proof scope: both interface globals are nonnull; their null paths are
/// excluded. Helpers are scripted. Record-pointer identity and scratch-pointer
/// identity are skipped while declared initialized pointee fields are observed.
/// Return comparison and full data-section scanning are disabled. Zero stack
/// fill pins record padding and fields the native wrapper leaves unwritten.
///
/// Return when the owner already has a face; otherwise query dimensions,
/// initialize the submission record, submit it, and mark the owner complete.
unsafe fn ped_face(owner_value: u32) {
    let owner = owner_value as *mut u8;
    let ready = unsafe { owner.add(OWNER_READY_OFFSET).cast::<u32>().read_unaligned() };
    if ready != 0 {
        return;
    }

    let mut width = DEFAULT_DIMENSION;
    let mut height = DEFAULT_DIMENSION;
    let first_owner = unsafe { lf_checker_rt::global::<u32>(FIRST_OWNER_FILE_VA).read_unaligned() };
    if first_owner != 0 {
        width = unsafe { vcall0(first_owner, WIDTH_SLOT) };
        let second_owner = unsafe { lf_checker_rt::global::<u32>(FIRST_OWNER_FILE_VA).read_unaligned() };
        if second_owner != 0 {
            height = unsafe { vcall0(second_owner, HEIGHT_SLOT) };
        }
    }

    let mut setup_context = [0u32; 1];
    let _ = callee_thiscall!(3, u32, setup_context.as_mut_ptr() as u32, 0);

    let final_owner = unsafe { lf_checker_rt::global::<u32>(FINAL_OWNER_FILE_VA).read_unaligned() };
    let mut record = [0u8; 0x30];
    record[0x08..0x0c].copy_from_slice(&1u32.to_ne_bytes());
    record[0x0c..0x10].copy_from_slice(&1u32.to_ne_bytes());
    record[0x10] = 1;
    record[0x25] = 1;
    record[0x2c..0x30].copy_from_slice(&2u32.to_ne_bytes());
    let result = unsafe {
        vcall_submit(
            final_owner,
            lf_checker_rt::relocated(FINAL_NAME_FILE_VA),
            FINAL_COUNT,
            width,
            height,
            FINAL_SPAN,
            record.as_mut_ptr() as u32,
        )
    };
    unsafe {
        owner.add(OWNER_READY_OFFSET).cast::<u32>().write_unaligned(result);
        owner.add(OWNER_DONE_OFFSET).write(1);
    }
}

export!(thiscall, rw_00ca27f0(this: u32) -> () {
    unsafe { ped_face(this) }
});
