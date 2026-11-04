use anyhow::Result;
use compositor::{ComponentStatus, Config, Compositor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_lifecycle_aaa() -> Result<()> {
        let config = Config::default();
        let mut comp = Compositor::new(config);
        
        assert_eq!(comp.status(), ComponentStatus::Stopped);
        comp.initialize()?;
        comp.start()?;
        assert_eq!(comp.status(), ComponentStatus::Running);
        comp.stop()?;
        assert_eq!(comp.status(), ComponentStatus::Stopped);
        Ok(())
    }

    #[test]
    fn test_compositor_metadata_aaa() {
        let config = Config::default();
        let comp = Compositor::new(config);
        assert_eq!(comp.name(), "SigmaOS-compositor");
        assert_eq!(comp.version(), "0.1.0");
    }
}
