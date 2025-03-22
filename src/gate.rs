pub const fn nand(inputs: &[bool]) -> bool {
    let mut i = 0;
    while i < inputs.len() {
        if !inputs[i] {
            return true;
        }
        i += 1;
    }
    false
}

pub const fn and(inputs: &[bool]) -> bool {
    not(nand(inputs))
}

pub const fn not(input: bool) -> bool {
    nand(&[input, input])
}

pub const fn or<const N: usize>(inputs: &[bool; N]) -> bool {
    let mut inverted = [false; N];
    let mut i = 0;
    while i < inputs.len() {
        inverted[i] = not(inputs[i]);
        i += 1;
    }

    nand(&inverted)
}

pub const fn nor<const N: usize>(inputs: &[bool; N]) -> bool {
    not(or(inputs))
}

pub const fn xor<const N: usize>(inputs: &[bool; N]) -> bool {
    not(xnor(inputs))
}

pub const fn xnor<const N: usize>(inputs: &[bool; N]) -> bool {
    let mut inverted = [false; N];
    let mut i = 0;
    while i < N {
        inverted[i] = not(inputs[i]);
        i += 1;
    }
    nand(&[nand(&inverted), nand(inputs)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        for (inputs, expected) in [
            (&[false][..], true),
            (&[true][..], false),
            (&[false, false][..], true),
            (&[false, true][..], true),
            (&[true, true][..], false),
            (&[true, false, false][..], true),
            (&[true, true, true][..], false),
        ] {
            assert_eq!(nand(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_and() {
        for (inputs, expected) in [
            (&[false, false][..], false),
            (&[false, true][..], false),
            (&[true, true][..], true),
            (&[true, false, false][..], false),
            (&[true, true, true][..], true),
        ] {
            assert_eq!(and(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_not() {
        for (input, expected) in [(true, false), (false, true)] {
            assert_eq!(not(input), expected, "failed for input: {:?}", input)
        }
    }

    #[test]
    fn test_or() {
        for (inputs, expected) in [
            (&[false, false], false),
            (&[false, true], true),
            (&[true, true], true),
        ] {
            assert_eq!(or(inputs), expected, "failed for inputs: {:?}", inputs)
        }

        for (inputs, expected) in [(&[true, false, false], true), (&[true, true, true], true)] {
            assert_eq!(or(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_nor() {
        for (inputs, expected) in [
            (&[false, false], true),
            (&[false, true], false),
            (&[true, true], false),
        ] {
            assert_eq!(nor(inputs), expected, "failed for inputs: {:?}", inputs)
        }

        for (inputs, expected) in [(&[true, false, false], false), (&[true, true, true], false)] {
            assert_eq!(nor(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_xor() {
        for (inputs, expected) in [
            (&[false, true], true),
            (&[true, true], false),
            (&[false, false], false),
        ] {
            assert_eq!(xor(inputs), expected, "failed for inputs: {:?}", inputs)
        }

        for (inputs, expected) in [(&[true, false, false], true), (&[true, true, true], false)] {
            assert_eq!(xor(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_xnor() {
        for (inputs, expected) in [
            (&[false, false], true),
            (&[false, true], false),
            (&[true, true], true),
        ] {
            assert_eq!(xnor(inputs), expected, "failed for inputs: {:?}", inputs)
        }

        for (inputs, expected) in [(&[true, false, false], false), (&[true, true, true], true)] {
            assert_eq!(xnor(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }
}
