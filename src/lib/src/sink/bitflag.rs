#[macro_export]
macro_rules! bitflag {
    ($target:ident { $($n:expr => $name:ident $desc:expr $(,)?)* }) => {
        ::paste::paste! {
            $(
                #[doc = concat!(stringify!($target), ": ", $desc)]
                #[allow(dead_code)]
                pub const [< $name:upper >]: u32 = 1 << $n;
            )*

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct $target {
                cap: u32,
            }

            #[allow(dead_code)]
            impl $target {
                $(
                    #[doc = "Check support for " $desc]
                    pub fn [<can_ $name:lower >](&self) -> bool {
                        (self.cap & [< $name:upper >]) != 0
                    }

                    #[doc = "Enable support for " $desc]
                    pub fn [<enable_ $name:lower >](&mut self) {
                        self.cap |= [< $name:upper >];
                    }

                    #[doc = "Disable support for " $desc]
                    pub fn [<disable_ $name:lower >](&mut self) {
                        self.cap &= ![< $name:upper >];
                    }
                    #[doc = "Create with support for " $desc]
                    pub fn [<with_ $name:lower >](mut self) -> Self {
                        self.cap |= [< $name:upper >];
                        self
                    }
                )*

                pub fn new() -> Self {
                    Self::default()
                }

                /// Check if it can any of the given capabilities
                ///
                /// Example: `check_any(DO_1 | DO_2)` // returns `true` if `DO_1` or `DO_2` are enabled
                pub fn check_any(&self, cap: u32) -> bool {
                    (self.cap & cap) != 0
                }

                /// Check if it can all of the given capabilities
                ///
                /// Example: `check_all(DO_1 | DO_2)` // returns `true` if `DO_1` and `DO_2` are both enabled
                pub fn check_all(&self, cap: u32) -> bool {
                    (self.cap & cap) == cap
                }
            }

            impl From<u32> for $target {
                fn from(cap: u32) -> Self {
                    Self { cap }
                }
            }

            impl From<$target> for u32 {
                fn from(value: $target) -> Self {
                    value.cap
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    bitflag!(Test {
        0 => do_1 "can do 1",
        1 => do_2 "can do 2",
        2 => do_3 "can do 3",
    });

    #[test]
    fn test_bitflag() {
        let mut test = Test::default();
        assert!(!test.can_do_1());
        test.enable_do_1();
        assert!(test.can_do_1());

        let test = Test::new().with_do_1().with_do_2();
        assert!(test.check_any(DO_2));
        assert!(test.check_any(DO_1 | DO_3));
        assert!(!test.check_any(DO_3));
        assert!(test.check_all(DO_1));
        assert!(test.check_all(DO_1 | DO_2));
        assert!(!test.check_all(DO_1 | DO_3));
    }
}
