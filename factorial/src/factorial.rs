    pub trait Factorial {
        fn fact(self) -> Self;
    }

    impl Factorial for u8 {
        fn fact(self) -> Self {
            // Now we introduce a condition
            if self == 0 {
                1
            } else {
                self * Self::fact(self - 1)
            }
        }
    }

    impl Factorial for u16 {
        fn fact(self) -> Self {
            // Now we introduce a condition
            if self == 0 {
                1
            } else {
                self * Self::fact(self - 1)
            }
        }
    }

    impl Factorial for u32 {
        fn fact(self) -> Self {
            // Now we introduce a condition
            if self == 0 {
                1
            } else {
                self * Self::fact(self - 1)
            }
        }
    }
