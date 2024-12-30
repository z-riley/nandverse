package gates

// NAND performs the function of a NAND logic gate.
func NAND(in ...bool) bool {
	if len(in) == 0 {
		panic("undefined behaviour")
	}

	for _, input := range in {
		if !input {
			return true
		}
	}
	return false
}

// NOT performs the function of a NOT logic gate.
func NOT(in bool) bool {
	return NAND(in)
}

// AND performs the function of an AND logic gate.
func AND(in ...bool) bool {
	return NOT(NAND(in...))
}

// OR performs the function of an OR logic gate.
func OR(in ...bool) bool {
	panic("todo")
}

// NOR performs the function of a NOR logic gate.
func NOR(in ...bool) bool {
	panic("todo")
}

// XOR performs the function of an XOR logic gate.
func XOR(in ...bool) bool {
	panic("todo")
}

// XNOR performs the function of an XNOR logic gate.
func XNOR(in ...bool) bool {
	panic("todo")
}
