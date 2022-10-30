/// Extension trait to provide additional methods on integers.
pub trait NumExt {
    fn next_multiple_of_tmp(self, rhs: Self) -> Self;
}

impl NumExt for i32 {
    /**
    Access to nightly-only method without requiring its toolchain.

    See tracking issue https://github.com/rust-lang/rust/issues/88581
    */
    fn next_multiple_of_tmp(self, rhs: Self) -> Self {
        // This would otherwise fail when calculating `r` when self == T::MIN.
        if rhs == -1 {
            return self;
        }

        let r = self % rhs;

        let m = if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
            r + rhs
        } else {
            r
        };

        if m == 0 {
            self
        } else {
            self + (rhs - m)
        }
    }
}
