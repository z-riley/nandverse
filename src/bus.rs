use core::ops::BitOrAssign;
use num::PrimInt;

pub fn bus_to_u8(bits: [bool; 8]) -> u8 {
    bus_to_num(&bits)
}

pub fn bus_to_u16(bits: [bool; 16]) -> u16 {
    bus_to_num(&bits)
}

pub fn bus_to_u32(bits: [bool; 32]) -> u32 {
    bus_to_num(&bits)
}

pub fn bus_to_u64(bits: [bool; 64]) -> u64 {
    bus_to_num(&bits)
}

pub fn bus_to_num<T>(bus: &[bool]) -> T
where
    T: PrimInt + BitOrAssign<T>,
{
    let mut val: T = T::from(0).unwrap();

    for (i, bit) in bus.iter().enumerate() {
        val |= if *bit {
            T::from(1).unwrap() << i
        } else {
            T::from(0).unwrap()
        };
    }

    val
}

pub fn u8_to_bus(value: u8) -> [bool; 8] {
    to_bus(value)
}

pub fn u16_to_bus(value: u16) -> [bool; 16] {
    to_bus(value)
}

pub fn u32_to_bus(value: u32) -> [bool; 32] {
    to_bus(value)
}

pub fn u64_to_bus(value: u64) -> [bool; 64] {
    to_bus(value)
}

pub fn to_bus<const N: usize, T>(value: T) -> [bool; N]
where
    T: PrimInt,
{
    if std::mem::size_of::<T>() > N {
        unreachable!("value doesn't fit into bus");
    }

    let mut bits = [false; N];
    (0..N).for_each(|i| {
        bits[i] = ((value >> i) & T::from(1).unwrap()) == T::from(1).unwrap();
    });

    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_val() {
        assert_eq!(bus_to_u32([true; 32]), u32::MAX);
        assert_eq!(
            bus_to_u8([true, false, true, false, false, false, false, false]),
            5
        );
    }

    #[test]
    fn test_to_bus() {
        let n = 64;
        assert_eq!(bus_to_u8(u8_to_bus(n)), n);
    }
}
