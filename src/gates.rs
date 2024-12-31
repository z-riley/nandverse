pub fn nand(inputs: &[bool]) -> bool {
    if inputs.len() < 2 {
        panic!("undefined behaviour");
    }

    !inputs.iter().all(|&x| x)
}

pub fn and(inputs: &[bool]) -> bool {
    not(&nand(inputs))
}

pub fn not(input: &bool) -> bool {
    nand(&[*input, *input])
}

pub fn or(inputs: &[bool]) -> bool {
    nand(&inputs.iter().map(not).collect::<Vec<bool>>())
}

pub fn nor(inputs: &[bool]) -> bool {
    not(&or(inputs))
}

pub fn xor(inputs: &[bool]) -> bool {
    not(&xnor(inputs))
}

pub fn xnor(inputs: &[bool]) -> bool {
    nand(&[
        nand(&inputs.iter().map(not).collect::<Vec<bool>>()),
        nand(inputs),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        for (inputs, expected) in [
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
        for (input, expected) in [(&true, false), (&false, true)] {
            assert_eq!(not(input), expected, "failed for input: {:?}", input)
        }
    }

    #[test]
    fn test_or() {
        for (inputs, expected) in [
            (&[false, false][..], false),
            (&[false, true][..], true),
            (&[true, true][..], true),
            (&[true, false, false][..], true),
            (&[true, true, true][..], true),
        ] {
            assert_eq!(or(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_nor() {
        for (inputs, expected) in [
            (&[false, false][..], true),
            (&[false, true][..], false),
            (&[true, true][..], false),
            (&[true, false, false][..], false),
            (&[true, true, true][..], false),
        ] {
            assert_eq!(nor(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_xor() {
        for (inputs, expected) in [
            (&[false, true][..], true),
            (&[true, true][..], false),
            (&[false, false][..], false),
            (&[true, false, false][..], true),
            (&[true, true, true][..], false),
        ] {
            assert_eq!(xor(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }

    #[test]
    fn test_xnor() {
        for (inputs, expected) in [
            (&[false, false][..], true),
            (&[false, true][..], false),
            (&[true, true][..], true),
            (&[true, false, false][..], false),
            (&[true, true, true][..], true),
        ] {
            assert_eq!(xnor(inputs), expected, "failed for inputs: {:?}", inputs)
        }
    }
}
