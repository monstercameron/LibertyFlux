use lf_checker_rt::{callee_thiscall, export};

const STREAM_TARGET_FILE_VA: u32 = 0x01305d30;
const PARAMETER_FLAGS_OFFSET: usize = 0x20;

#[repr(C, align(16))]
struct VectorBuffer([u32; 4]);

#[repr(C, align(16))]
struct RequestRecord([u8; 0x54]);

#[inline(always)]
unsafe fn byte_at(base: u32, offset: usize) -> u8 {
    unsafe { (base as *const u8).add(offset).read() }
}

#[inline(always)]
unsafe fn word_at(base: u32, offset: usize) -> u32 {
    unsafe { ((base as *const u8).add(offset) as *const u32).read_unaligned() }
}

/// Returns for a null parameter block. Otherwise records three helper vectors,
/// owner scalar fields, two normalized flags, and a conditional parameter flag.
/// The vector helpers each write three words; their fourth words are allocated
/// for aligned access but remain outside snapshots because the native helpers
/// leave them unwritten. The adjacent flag byte is fixed by the proof's zero
/// stack fill and is recorded with the three defined flag bytes.
unsafe fn emit(this: u32, params: u32, wrong: bool) {
    if params == 0 {
        return;
    }

    let mut vector_a = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(1, u32, this, vector_a.0.as_mut_ptr() as u32);

    let mut vector_b = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(2, u32, this, vector_b.0.as_mut_ptr() as u32);

    let mut vector_c = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(3, u32, this, vector_c.0.as_mut_ptr() as u32);
    if wrong {
        vector_b.0[0] = vector_b.0[0].wrapping_add(1);
    }

    let owner_flags = unsafe { byte_at(this, PARAMETER_FLAGS_OFFSET) };
    let parameter_flags = unsafe { word_at(params, 0x28) };
    let conditional_flag = if parameter_flags & 0x3c0 == 0x80
        && unsafe { word_at(params, 0x1304) } == 4
    {
        1
    } else {
        0
    };

    let mut record = RequestRecord([0; 0x54]);
    let bytes = record.0.as_mut_ptr();
    unsafe {
        bytes.cast::<u32>().write_unaligned(params);
        for (index, value) in vector_a.0[..3].iter().copied().enumerate() {
            bytes.add(0x10 + index * 4).cast::<u32>().write_unaligned(value);
        }
        for (index, value) in vector_b.0[..3].iter().copied().enumerate() {
            bytes.add(0x20 + index * 4).cast::<u32>().write_unaligned(value);
        }
        for (index, value) in vector_c.0[..3].iter().copied().enumerate() {
            bytes.add(0x30 + index * 4).cast::<u32>().write_unaligned(value);
        }
        bytes.add(0x40).cast::<u32>().write_unaligned(word_at(this, 0x28));
        bytes.add(0x44).cast::<u32>().write_unaligned(
            ((this as *const u8).add(0x18) as *const u16).read_unaligned() as u32,
        );
        bytes.add(0x48).cast::<u32>().write_unaligned(u32::from(owner_flags & 0x1f));
        bytes.add(0x4c).cast::<u32>().write_unaligned(word_at(this, 0x1c));
        bytes.add(0x50).write((owner_flags >> 6) & 1);
        bytes.add(0x51).write(owner_flags >> 7);
        bytes.add(0x52).write(conditional_flag);
    }

    let _ = callee_thiscall!(
        4,
        u32,
        lf_checker_rt::relocated(STREAM_TARGET_FILE_VA),
        bytes as u32,
        1
    );
}

export!(thiscall, rw_00c049f0(this: u32, params: u32) -> () {
    unsafe { emit(this, params, false) }
});

