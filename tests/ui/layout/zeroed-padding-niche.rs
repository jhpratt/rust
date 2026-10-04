//@ revisions: unoptimized optimized
//@[unoptimized] compile-flags: -Copt-level=0
//@[optimized] compile-flags: -O
//@ ignore-endian-big
//@ run-pass

#![allow(dead_code)]

#[repr(zeroed_padding)]
struct Padded {
    byte: u8,
    word: u16,
}

#[repr(zeroed_padding, align(4))]
struct Aligned {
    byte: u8,
    word: u16,
}

enum PaddedNiche {
    Empty,
    Value(Padded),
}

enum AlignedNiche {
    Empty,
    Value(Aligned),
}

#[repr(zeroed_padding, u32, align(8))]
enum ExplicitlyRepr {
    Empty,
    Value { byte: u8, word: u16 },
}

#[repr(zeroed_padding)]
enum PaddedEnum {
    Empty,
    Value { byte: u8, word: u16 },
}

fn main() {
    assert_eq!(size_of::<PaddedNiche>(), size_of::<Padded>());
    assert_eq!(size_of::<AlignedNiche>(), size_of::<Aligned>());

    let value = Padded { byte: 1, word: 0x2345 };
    let bytes: [u8; 4] = unsafe { std::mem::transmute(value) };
    assert_eq!(bytes, [0x45, 0x23, 1, 0]);

    let value = Aligned { byte: 2, word: 0x3456 };
    let bytes: [u8; 4] = unsafe { std::mem::transmute(value) };
    assert_eq!(bytes, [0x56, 0x34, 2, 0]);

    let value = ExplicitlyRepr::Value { byte: 3, word: 0x4567 };
    let bytes: [u8; 8] = unsafe { std::mem::transmute(value) };
    assert_eq!(bytes, [1, 0, 0, 0, 3, 0, 0x67, 0x45]);

    let value = PaddedEnum::Value { byte: 4, word: 0x5678 };
    let bytes: [u8; 4] = unsafe { std::mem::transmute(value) };
    assert_eq!(bytes, [1, 4, 0x78, 0x56]);
}
