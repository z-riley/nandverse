use core::ops::BitOrAssign;
use num::PrimInt;

#[derive(Clone, Copy, Debug)]
/// Represents a collection of parallel signals. Can be read or written to as an integer value
pub struct Bus<const N: usize> {
    bits: [bool; N],
}

impl<const N: usize> TryFrom<u8> for Bus<N> {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        bus_from_num(value)
    }
}

impl<const N: usize> TryFrom<u16> for Bus<N> {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        bus_from_num(value)
    }
}

impl<const N: usize> TryFrom<u32> for Bus<N> {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        bus_from_num(value)
    }
}

impl<const N: usize> TryFrom<i8> for Bus<N> {
    type Error = &'static str;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        bus_from_num(value)
    }
}

impl<const N: usize> TryFrom<i16> for Bus<N> {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        bus_from_num(value)
    }
}

impl<const N: usize> TryFrom<i32> for Bus<N> {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        bus_from_num(value)
    }
}

fn bus_from_num<const N: usize, T: PrimInt>(value: T) -> Result<Bus<N>, &'static str> {
    if !fits_in_bits(value, N) {
        return Err("value doesn't fit into bus");
    }

    let mut bits = [false; N];
    (0..N.min(u32::BITS as usize)).for_each(|i| {
        bits[i] = (value >> i & T::from(1).unwrap()) == T::from(1).unwrap();
    });

    Ok(Bus { bits })
}

/// Returns whether type T can fit into n bits
fn fits_in_bits<T>(_: T, n: usize) -> bool {
    std::mem::size_of::<T>() * 8 <= n
}

impl<const N: usize> TryFrom<Bus<N>> for u8 {
    type Error = &'static str;

    fn try_from(value: Bus<N>) -> Result<Self, Self::Error> {
        num_from_bus(value)
    }
}

impl<const N: usize> TryFrom<Bus<N>> for u16 {
    type Error = &'static str;

    fn try_from(value: Bus<N>) -> Result<Self, Self::Error> {
        num_from_bus(value)
    }
}

impl<const N: usize> TryFrom<Bus<N>> for u32 {
    type Error = &'static str;

    fn try_from(value: Bus<N>) -> Result<Self, Self::Error> {
        num_from_bus(value)
    }
}

impl<const N: usize> TryFrom<Bus<N>> for i8 {
    type Error = &'static str;

    fn try_from(value: Bus<N>) -> Result<Self, Self::Error> {
        num_from_bus(value)
    }
}

impl<const N: usize> TryFrom<Bus<N>> for i16 {
    type Error = &'static str;

    fn try_from(value: Bus<N>) -> Result<Self, Self::Error> {
        num_from_bus(value)
    }
}

impl<const N: usize> TryFrom<Bus<N>> for i32 {
    type Error = &'static str;

    fn try_from(value: Bus<N>) -> Result<Self, Self::Error> {
        num_from_bus(value)
    }
}

fn num_from_bus<const N: usize, T: PrimInt + BitOrAssign<T>>(
    bus: Bus<N>,
) -> Result<T, &'static str> {
    let mut val: T = T::from(0).unwrap();

    if !fits_in_type(N, val) {
        return Err("bus doesn't fit into value");
    }

    for (i, bit) in bus.bits.iter().enumerate() {
        val |= if *bit {
            T::from(1).unwrap() << i
        } else {
            T::from(0).unwrap()
        };
    }

    Ok(val)
}

/// Returns whether n bits can fit into type T  
fn fits_in_type<T>(n: usize, _: T) -> bool {
    n <= std::mem::size_of::<T>() * 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fits() {
        assert!(fits_in_bits(1u8, 8));
        assert!(!fits_in_bits(1u32, 31));
        assert!(!fits_in_bits(-1i64, 60));
    }

    #[test]
    fn test_try_from_u32() {
        let bus = Bus::<32>::try_from(0);
        assert!(bus.is_ok());

        let bus = Bus::<6>::try_from(255);
        assert!(bus.is_err());
    }

    #[test]
    fn test_try_from_bus() {
        // Bus into number
        let bus = Bus { bits: [true; 32] };
        let val: Result<u32, _> = bus.try_into();
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), u32::MAX);

        // Bus too large for number type
        let bus = Bus { bits: [true; 16] };
        let val: Result<u8, _> = bus.try_into();
        assert!(val.is_err());

        // Bus from number
        let bus = Bus::<8>::try_from(101u8).unwrap();
        let val = u8::try_from(bus);
        assert!(val.is_ok());

        // Check value
        let val: u32 = bus.try_into().unwrap();
        assert_eq!(val, 101);
    }
}
