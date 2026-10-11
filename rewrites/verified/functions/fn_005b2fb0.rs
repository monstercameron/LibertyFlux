// original: 0x005B2FB0 blend_menu_color_set
/// Specification: choose the menu-color channel from the dispatch key,
/// preserve the other bytes of each existing color, and replace eight
/// selected bytes with scaled helper results. The low parameter byte selects
/// one of two groups of eight global color words.

const C_PAIR_LOOKUP: u32 = 1;
const C_COLOR_SELECT: u32 = 2;
const C_RANDOM_VALUE: u32 = 3;

const G_D10: u32 = 0x01160D10;
const G_D14: u32 = 0x01160D14;
const G_D18: u32 = 0x01160D18;
const G_D1C: u32 = 0x01160D1C;
const G_D20: u32 = 0x01160D20;
const G_D2C: u32 = 0x01160D2C;
const G_D30: u32 = 0x01160D30;
const G_D34: u32 = 0x01160D34;
const G_D38: u32 = 0x01160D38;
const G_D3C: u32 = 0x01160D3C;
const G_D44: u32 = 0x01160D44;
const G_D58: u32 = 0x01160D58;
const G_D5C: u32 = 0x01160D5C;
const G_D60: u32 = 0x01160D60;
const G_D64: u32 = 0x01160D64;
const G_D68: u32 = 0x01160D68;
const G_SCALE: u32 = 0x00FE8680;
const G_DISPATCH: u32 = 0x011D6FD4;

#[inline(always)]
unsafe fn read_global_word(file_va: u32) -> u32 {
    // SAFETY: the checker maps the original image and relocates this global.
    unsafe { core::ptr::read_volatile(lf_checker_rt::global::<u32>(file_va)) }
}

#[inline(always)]
unsafe fn read_global_byte(file_va: u32) -> u8 {
    let aligned = file_va & !3;
    let shift = (file_va & 3) * 8;
    // SAFETY: aligned is a word within the mapped image's data section.
    unsafe { (read_global_word(aligned) >> shift) as u8 }
}

#[inline(always)]
unsafe fn write_global_word(file_va: u32, value: u32) {
    // SAFETY: the contract declares each written word separately.
    unsafe { core::ptr::write_volatile(lf_checker_rt::global::<u32>(file_va), value) }
}

#[inline(always)]
fn pack_color(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (u32::from(a) << 24) | (u32::from(b) << 16) | (u32::from(c) << 8) | u32::from(d)
}

#[inline(always)]
fn cvt_trunc(value: f32) -> u32 {
    if value.is_nan() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
        0x8000_0000
    } else {
        value as i32 as u32
    }
}

#[inline(always)]
fn scaled_channel(random_word: u32, multiplier: u32, scale: f32) -> u8 {
    let random_integer = (random_word & 0xFFFF) as i32;
    let random_float = core::hint::black_box(random_integer as f32);
    let scale = core::hint::black_box(scale);
    let scaled_random = core::hint::black_box(random_float * scale);
    let multiplier_float = core::hint::black_box(multiplier as f32);
    let channel = core::hint::black_box(scaled_random * multiplier_float);
    cvt_trunc(channel) as u8
}

#[inline(always)]
unsafe fn lookup_channel(key: u32, flag: u32, lane: u32, scale: f32) -> u8 {
    let count = lf_checker_rt::callee_fastcall!(C_PAIR_LOOKUP, u32, key, flag, lane) as u8;
    let random = lf_checker_rt::callee_cdecl!(C_RANDOM_VALUE, u32,);
    scaled_channel(random, u32::from(count), scale)
}

#[inline(always)]
unsafe fn select_channel(key: u32, parameter: u32, lane: u32, scale: f32) -> u8 {
    let count = lf_checker_rt::callee_cdecl!(C_COLOR_SELECT, u32, key, parameter, lane) as u8;
    let random = lf_checker_rt::callee_cdecl!(C_RANDOM_VALUE, u32,);
    scaled_channel(random, u32::from(count) + 1, scale)
}

unsafe fn initialize_frame(parameter: u32) -> [u8; 0x7C] {
    let mut frame = [0; 0x7C];
    let flag = parameter as u8;

    if flag == 0 {
        frame[0x20] = unsafe { read_global_byte(G_D58 + 3) };
        frame[0x0C] = unsafe { read_global_byte(G_D58 + 2) };
        frame[0x0D] = unsafe { read_global_byte(G_D58 + 3) };
        frame[0x24] = unsafe { read_global_byte(G_D58 + 1) };
        frame[0x28] = unsafe { read_global_byte(G_D58) };
        frame[0x2C] = unsafe { read_global_byte(G_D10 + 3) };
        frame[0x0E] = unsafe { read_global_byte(G_D10 + 2) };
        frame[0x0F] = unsafe { read_global_byte(G_D10 + 3) };
        frame[0x34] = unsafe { read_global_byte(G_D10) };
        frame[0x30] = unsafe { read_global_byte(G_D10 + 1) };
        frame[0x10] = unsafe { read_global_byte(G_D14 + 2) };
        frame[0x11] = unsafe { read_global_byte(G_D14 + 3) };
        frame[0x38] = unsafe { read_global_byte(G_D14 + 3) };
        frame[0x40] = unsafe { read_global_byte(G_D14) };
        frame[0x3C] = unsafe { read_global_byte(G_D14 + 1) };
        frame[0x12] = unsafe { read_global_byte(G_D18 + 2) };
        frame[0x13] = unsafe { read_global_byte(G_D18 + 3) };
        frame[0x44] = unsafe { read_global_byte(G_D18 + 3) };
        frame[0x4C] = unsafe { read_global_byte(G_D18) };
        frame[0x48] = unsafe { read_global_byte(G_D18 + 1) };
        frame[0x14] = unsafe { read_global_byte(G_D5C + 2) };
        frame[0x15] = unsafe { read_global_byte(G_D5C + 3) };
        frame[0x50] = unsafe { read_global_byte(G_D5C + 3) };
        frame[0x58] = unsafe { read_global_byte(G_D5C) };
        frame[0x54] = unsafe { read_global_byte(G_D5C + 1) };
        frame[0x16] = unsafe { read_global_byte(G_D64 + 2) };
        frame[0x17] = unsafe { read_global_byte(G_D64 + 3) };
        frame[0x5C] = unsafe { read_global_byte(G_D64 + 3) };
        frame[0x64] = unsafe { read_global_byte(G_D64) };
        frame[0x60] = unsafe { read_global_byte(G_D64 + 1) };
        frame[0x18] = unsafe { read_global_byte(G_D1C + 2) };
        frame[0x19] = unsafe { read_global_byte(G_D1C + 3) };
        frame[0x68] = unsafe { read_global_byte(G_D1C + 3) };
        frame[0x70] = unsafe { read_global_byte(G_D1C) };
        frame[0x6C] = unsafe { read_global_byte(G_D1C + 1) };
        frame[0x1A] = unsafe { read_global_byte(G_D20 + 2) };
        frame[0x1B] = unsafe { read_global_byte(G_D20 + 3) };
        frame[0x1C] = unsafe { read_global_byte(G_D20 + 1) };
        frame[0x74] = unsafe { read_global_byte(G_D20 + 3) };
        frame[0x78] = unsafe { read_global_byte(G_D20) };
    } else {
        frame[0x20] = unsafe { read_global_byte(G_D44 + 3) };
        frame[0x0C] = unsafe { read_global_byte(G_D44 + 2) };
        frame[0x24] = unsafe { read_global_byte(G_D44 + 1) };
        frame[0x28] = unsafe { read_global_byte(G_D44) };
        frame[0x2C] = unsafe { read_global_byte(G_D2C + 3) };
        frame[0x0E] = unsafe { read_global_byte(G_D2C + 2) };
        frame[0x30] = unsafe { read_global_byte(G_D2C + 1) };
        frame[0x34] = unsafe { read_global_byte(G_D2C) };
        frame[0x38] = unsafe { read_global_byte(G_D30 + 3) };
        frame[0x10] = unsafe { read_global_byte(G_D30 + 2) };
        frame[0x3C] = unsafe { read_global_byte(G_D30 + 1) };
        frame[0x40] = unsafe { read_global_byte(G_D30) };
        frame[0x44] = unsafe { read_global_byte(G_D34 + 3) };
        frame[0x12] = unsafe { read_global_byte(G_D34 + 2) };
        frame[0x48] = unsafe { read_global_byte(G_D34 + 1) };
        frame[0x4C] = unsafe { read_global_byte(G_D34) };
        frame[0x50] = unsafe { read_global_byte(G_D60 + 3) };
        frame[0x14] = unsafe { read_global_byte(G_D60 + 2) };
        frame[0x54] = unsafe { read_global_byte(G_D60 + 1) };
        frame[0x58] = unsafe { read_global_byte(G_D60) };
        frame[0x5C] = unsafe { read_global_byte(G_D68 + 3) };
        frame[0x16] = unsafe { read_global_byte(G_D68 + 2) };
        frame[0x60] = unsafe { read_global_byte(G_D68 + 1) };
        frame[0x64] = unsafe { read_global_byte(G_D68) };
        frame[0x68] = unsafe { read_global_byte(G_D38 + 3) };
        frame[0x18] = unsafe { read_global_byte(G_D38 + 2) };
        frame[0x6C] = unsafe { read_global_byte(G_D38 + 1) };
        frame[0x70] = unsafe { read_global_byte(G_D38) };
        frame[0x74] = unsafe { read_global_byte(G_D3C + 3) };
        frame[0x1A] = unsafe { read_global_byte(G_D3C + 2) };
        frame[0x1C] = unsafe { read_global_byte(G_D3C + 1) };
        frame[0x78] = unsafe { read_global_byte(G_D3C) };
    }

    frame
}

unsafe fn run(parameter: u32, mutant: bool) {
    let selector = unsafe { read_global_word(G_DISPATCH) };
    let flag = u32::from(parameter as u8);
    let scale = unsafe { core::ptr::read_volatile(lf_checker_rt::global::<f32>(G_SCALE)) };
    let mut frame = unsafe { initialize_frame(parameter) };

    let write_offsets = match selector {
        1 => [0x24, 0x30, 0x3C, 0x48, 0x54, 0x60, 0x6C],
        2 => [0x0C, 0x0E, 0x10, 0x12, 0x14, 0x16, 0x18],
        3 => [0x20, 0x2C, 0x38, 0x44, 0x50, 0x5C, 0x68],
        _ => [0x28, 0x34, 0x40, 0x4C, 0x58, 0x64, 0x70],
    };
    let generated = [
        unsafe { lookup_channel(7, flag, selector, scale) },
        unsafe { lookup_channel(0, flag, selector, scale) },
        unsafe { lookup_channel(1, flag, selector, scale) },
        unsafe { lookup_channel(2, flag, selector, scale) },
        unsafe { lookup_channel(3, flag, selector, scale) },
        unsafe { lookup_channel(4, flag, selector, scale) },
        unsafe { select_channel(1, parameter, selector, scale) },
    ];
    for (offset, channel) in write_offsets.into_iter().zip(generated) {
        frame[offset] = channel;
    }
    let final_channel = unsafe { select_channel(0, parameter, selector, scale) };
    let final_tail_channel = if selector == 1 {
        final_channel
    } else {
        let final_offset = match selector {
            2 => 0x1A,
            3 => 0x74,
            _ => 0x78,
        };
        frame[final_offset] = final_channel;
        frame[0x1C]
    };

    let (first_va, first_value, rest) = if flag == 0 {
        (
            G_D58,
            pack_color(frame[0x20], frame[0x0C], frame[0x24], frame[0x28]),
            [
                (G_D10, pack_color(frame[0x2C], frame[0x0E], frame[0x30], frame[0x34])),
                (G_D14, pack_color(frame[0x38], frame[0x10], frame[0x3C], frame[0x40])),
                (G_D18, pack_color(frame[0x44], frame[0x12], frame[0x48], frame[0x4C])),
                (G_D5C, pack_color(frame[0x50], frame[0x14], frame[0x54], frame[0x58])),
                (G_D64, pack_color(frame[0x5C], frame[0x16], frame[0x60], frame[0x64])),
                (G_D1C, pack_color(frame[0x68], frame[0x18], frame[0x6C], frame[0x70])),
                (G_D20, pack_color(frame[0x74], frame[0x1A], final_tail_channel, frame[0x78])),
            ],
        )
    } else {
        (
            G_D44,
            pack_color(frame[0x20], frame[0x0C], frame[0x24], frame[0x28]),
            [
                (G_D2C, pack_color(frame[0x2C], frame[0x0E], frame[0x30], frame[0x34])),
                (G_D30, pack_color(frame[0x38], frame[0x10], frame[0x3C], frame[0x40])),
                (G_D34, pack_color(frame[0x44], frame[0x12], frame[0x48], frame[0x4C])),
                (G_D60, pack_color(frame[0x50], frame[0x14], frame[0x54], frame[0x58])),
                (G_D68, pack_color(frame[0x5C], frame[0x16], frame[0x60], frame[0x64])),
                (G_D38, pack_color(frame[0x68], frame[0x18], frame[0x6C], frame[0x70])),
                (G_D3C, pack_color(frame[0x74], frame[0x1A], final_tail_channel, frame[0x78])),
            ],
        )
    };

    let displaced_va = if flag == 0 { G_D5C } else { G_D44 + 4 };
    unsafe { write_global_word(if mutant { displaced_va } else { first_va }, first_value) };
    for (file_va, value) in rest {
        unsafe { write_global_word(file_va, value) };
    }
}

lf_checker_rt::export!(thiscall, au31_fn(parameter: u32) -> () {
    unsafe { run(parameter, false) }
});

lf_checker_rt::export!(thiscall, au31_fn_wrong(parameter: u32) -> () {
    unsafe { run(parameter, true) }
});
