//! A Verus proof of `octal_overflow` in src/printer.rs (P2R-49), for inputs of any length.
//!
//! Verus cannot build php2rust with its Mago dependencies, so this file holds a copy of
//! the function with the iterator chain written as a loop. A change to `octal_overflow`
//! must be made here too. The Kani proofs in src/printer.rs check the real function for
//! inputs of up to 6 bytes.
//!
//! Run with `scripts/prove.sh verus`, which caps memory and time.

use vstd::prelude::*;

verus! {

/// `0` to `7`.
pub open spec fn octal(byte: u8) -> bool {
    48 <= byte <= 55
}

/// How many octal digits start at `at`, at most 3: the digits PHP reads after `\`.
pub open spec fn run(raw: Seq<u8>, at: int) -> int {
    if 0 <= at < raw.len() && octal(raw[at]) {
        if at + 1 < raw.len() && octal(raw[at + 1]) {
            if at + 2 < raw.len() && octal(raw[at + 2]) {
                3
            } else {
                2
            }
        } else {
            1
        }
    } else {
        0
    }
}

/// The value of the `count` octal digits that start at `at`.
pub open spec fn value(raw: Seq<u8>, at: int, count: int) -> int
    decreases count,
{
    if count <= 0 {
        0
    } else {
        value(raw, at, count - 1) * 8 + (raw[at + count - 1] - 48)
    }
}

/// The rule the function must follow, stated without arithmetic: an escape is above
/// `\377` when it has three digits and the first is `4` to `7`. A backslash with no
/// digit escapes the next byte, so `\\400` is a backslash and then text.
pub open spec fn overflow_from(raw: Seq<u8>, i: int) -> bool
    decreases raw.len() + 1 - i,
{
    if i < 0 || i >= raw.len() {
        false
    } else if raw[i] != 92 {
        overflow_from(raw, i + 1)
    } else if run(raw, i + 1) == 0 {
        overflow_from(raw, i + 2)
    } else if run(raw, i + 1) == 3 && raw[i + 1] >= 52 {
        true
    } else {
        overflow_from(raw, i + 1 + run(raw, i + 1))
    }
}

/// The copy of `octal_overflow`. The proof shows that it follows `overflow_from` for
/// every input, that it never indexes out of range, and that its sum cannot overflow.
/// A Rust slice of bytes is at most `isize::MAX` long, which the precondition states, so
/// `i + 2` fits in a `usize`.
pub fn octal_overflow(raw: &[u8]) -> (found: bool)
    requires
        raw@.len() <= isize::MAX,
    ensures
        found == overflow_from(raw@, 0),
{
    let mut i: usize = 0;
    while i < raw.len()
        invariant
            raw@.len() <= isize::MAX,
            overflow_from(raw@, 0) == overflow_from(raw@, i as int),
        decreases raw@.len() + 2 - i,
    {
        if raw[i] != 92 {
            i += 1;
            continue;
        }
        // `raw[i + 1..].iter().take(3).take_while(octal)` and the fold over it.
        let mut count: usize = 0;
        let mut sum: u32 = 0;
        while count < 3 && i + 1 + count < raw.len() && 48 <= raw[i + 1 + count] && raw[i + 1
            + count] <= 55
            invariant
                i < raw@.len() <= isize::MAX,
                count <= 3,
                i + 1 + count <= raw@.len(),
                forall|k: int| 0 <= k < count ==> octal(#[trigger] raw@[i + 1 + k]),
                sum == value(raw@, i + 1, count as int),
                count == 0 ==> sum == 0,
                count == 1 ==> sum < 8,
                count == 2 ==> sum < 64,
                count == 3 ==> sum < 512,
            decreases 3 - count,
        {
            sum = sum * 8 + (raw[i + 1 + count] - 48) as u32;
            count += 1;
        }
        proof {
            // Name each digit the loop read, so the solver can match them to `run`.
            if count > 0 {
                assert(octal(raw@[i + 1 + 0]));
            }
            if count > 1 {
                assert(octal(raw@[i + 1 + 1]));
            }
            if count > 2 {
                assert(octal(raw@[i + 1 + 2]));
            }
        }
        assert(count == run(raw@, i + 1));
        if count == 0 {
            i += 2;
            continue;
        }
        proof {
            if count == 3 {
                let first = raw@[i + 1] - 48;
                let second = raw@[i + 2] - 48;
                let third = raw@[i + 3] - 48;
                assert(sum == first * 64 + second * 8 + third) by {
                    reveal_with_fuel(value, 4);
                }
                assert((sum > 255) == (raw@[i + 1] >= 52)) by (nonlinear_arith)
                    requires
                        sum == first * 64 + second * 8 + third,
                        first == raw@[i + 1] - 48,
                        0 <= first <= 7,
                        0 <= second <= 7,
                        0 <= third <= 7,
                ;
            }
        }
        if sum > 255 {
            return true;
        }
        i += 1 + count;
    }
    false
}

fn main() {
}

} // verus!
