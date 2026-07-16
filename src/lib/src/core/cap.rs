use paste::paste;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Capability {
    cap: u32,
}

macro_rules! CAP {
    ($($n:expr => $name:ident $desc:expr $(,)?)*) => {
        paste! {
            $(
                #[doc = "Capability to " $desc]
                pub const [< $name:upper >]: u32 = 1 << $n;
            )*

            impl Capability {
                $(
                    #[doc = "Check if it can " $desc]
                    pub fn [<can_ $name:lower >](&self) -> bool {
                        (self.cap & [< $name:upper >]) != 0
                    }

                    #[doc = "Enable it to " $desc]
                    pub fn [<enable_ $name:lower >](&mut self) {
                        self.cap |= [< $name:upper >];
                    }

                    #[doc = "Disable it to " $desc]
                    pub fn [<disable_ $name:lower >](&mut self) {
                        self.cap &= ![< $name:upper >];
                    }
                    #[doc = "Append capability to " $desc]
                    pub fn [<with_ $name:lower >](mut self) -> Self {
                        self.cap |= [< $name:upper >];
                        self
                    }
                )*
            }
        }
    };
}

CAP! {
    0 => network_access "communicate with other dsot instances",
    1 => disk_access "read/write files to disk",
    2 => full_disk_access "read/write files to any path",
}

impl Capability {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if it can any of the given capabilities
    pub fn check_any(&self, cap: Capability) -> bool {
        (self.cap & cap.cap) != 0
    }

    /// Check if it can all of the given capabilities
    pub fn check_all(&self, cap: Capability) -> bool {
        (self.cap & cap.cap) == cap.cap
    }
}

impl From<u32> for Capability {
    fn from(cap: u32) -> Self {
        Self { cap }
    }
}

impl From<Capability> for u32 {
    fn from(value: Capability) -> Self {
        value.cap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_capability() {
        let cap = Capability::new().with_network_access().with_disk_access();
        assert!(cap.can_disk_access());
        assert!(!cap.can_full_disk_access());
    }

    #[test]
    fn test_check_any() {
        let cap = Capability::new().with_network_access().with_disk_access();
        assert!(cap.check_any(Capability::new().with_network_access()));
        assert!(!cap.check_any(Capability::new().with_full_disk_access()));
    }

    #[test]
    fn test_check_all() {
        let cap = Capability::new().with_network_access().with_disk_access();
        assert!(cap.check_all(Capability::new().with_network_access().with_disk_access()));
        assert!(!cap.check_all(Capability::new().with_full_disk_access().with_disk_access()));
    }
}
