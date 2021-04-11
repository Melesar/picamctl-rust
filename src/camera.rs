pub mod picamctl {
    use std::io::Error;
    use std::process::{Command, Child};

    pub enum Camera {
        Real(CameraHandle),
        Fake,
    }

    impl Camera {

        #[cfg(pi)]
        pub fn new() -> Self {
            Camera::Real(CameraHandle::new())
        }

        #[cfg(not(pi))]
        pub fn new() -> Self {
            Camera::Fake
        }
        
        pub fn set_enabled(&mut self, is_enabled: bool) -> Result<(), Error> {
            if let Camera::Real(cam) = self {
                return cam.set_enabled(is_enabled);
            }

            Ok(())
        }

        pub fn is_enabled(&self) -> bool {
            if let Camera::Real(cam) = self {
                return cam.is_enabled();
            }

            false
        }
    }

    struct CameraHandle {
        process: Option<Child>,
    }

    impl CameraHandle {
        fn new () -> Self {
            CameraHandle {process: None}
        }

        fn set_enabled (&mut self, is_enabled: bool) -> Result<(), Error> {
            if self.process.is_some() && is_enabled == false {
                self.process.as_mut().unwrap().kill()?;
                self.process = None;
                return Ok(());
            } else if self.process.is_none() && is_enabled {
                let child = Command::new("motion").spawn()?;
                self.process = Some(child);
                return Ok(());
            }

            Ok(())
        }

        fn is_enabled (&self) -> bool {
            self.process.is_some()
        }
    }
}
