crate::bitflag!(Capability {
    0 => network_access "communicate with other dsot instances",
    1 => disk_access "read/write files to disk",
    2 => full_disk_access "read/write files to any path",
});

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
        assert!(cap.check_any(Capability::new().with_network_access().into()));
        assert!(!cap.check_any(Capability::new().with_full_disk_access().into()));
    }

    #[test]
    fn test_check_all() {
        let cap = Capability::new().with_network_access().with_disk_access();
        assert!(
            cap.check_all(
                Capability::new()
                    .with_network_access()
                    .with_disk_access()
                    .into()
            )
        );
        assert!(
            !cap.check_all(
                Capability::new()
                    .with_full_disk_access()
                    .with_disk_access()
                    .into()
            )
        );
    }
}
