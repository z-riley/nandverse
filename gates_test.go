package gates

import (
	"testing"
)

type testCase struct {
	input []bool
	want  bool
}

func TestNAND(t *testing.T) {
	for _, test := range []testCase{
		{
			input: []bool{false},
			want:  true,
		},
		{
			input: []bool{false, false},
			want:  true,
		},
		{
			input: []bool{false, true},
			want:  true,
		},
		{
			input: []bool{true, false},
			want:  true,
		},
		{
			input: []bool{true, true},
			want:  false,
		},
		{
			input: []bool{true, false, false},
			want:  true,
		},
		{
			input: []bool{true, true, true},
			want:  false,
		},
	} {
		got := NAND(test.input...)
		if got != test.want {
			t.Errorf("test input: %v wanted: %t, got: %t", test.input, test.want, got)
		}
	}
}

func TestAND(t *testing.T) {
	for _, test := range []testCase{
		{
			input: []bool{true},
			want:  true,
		},
		{
			input: []bool{false, false},
			want:  false,
		},
		{
			input: []bool{false, true},
			want:  false,
		},
		{
			input: []bool{true, false},
			want:  false,
		},
		{
			input: []bool{true, true},
			want:  true,
		},
		{
			input: []bool{true, false, false},
			want:  false,
		},
		{
			input: []bool{true, true, true},
			want:  true,
		},
	} {
		got := AND(test.input...)
		if got != test.want {
			t.Errorf("test input: %v wanted: %t, got: %t", test.input, test.want, got)
		}
	}
}
