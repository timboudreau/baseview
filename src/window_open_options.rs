use crate::Size;

/// The dpi scaling policy of the window
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowScalePolicy {
    /// Use the system's dpi scale factor
    SystemScaleFactor,
    /// Use the given dpi scale factor (e.g. `1.0` = 96 dpi)
    ScaleFactor(f64),
}

/// The options for opening a new window
pub struct WindowOpenOptions {
    pub title: String,

    /// The logical size of the window.
    ///
    /// These dimensions will be scaled by the scaling policy specified in `scale`. Mouse
    /// position will be passed back as logical coordinates.
    pub size: Size,

    /// The dpi scaling policy
    pub scale: WindowScalePolicy,

    /// If provided, then an OpenGL context will be created for this window. You'll be able to
    /// access this context through [crate::Window::gl_context].
    #[cfg(feature = "opengl")]
    pub gl_config: Option<crate::gl::GlConfig>,

    pub mac_os_options : Option<MacOSWindowOptions>,
}

impl Default for WindowOpenOptions {
    fn default() -> Self {
        Self {
            title: "Baseview window".into(),
            size: Size::new(800., 600.),
            scale: WindowScalePolicy::SystemScaleFactor,
            #[cfg(feature = "opengl")]
            gl_config: Default::default(),
            mac_os_options: Default::default()
        }
    }
}

impl WindowOpenOptions {
    pub fn with_title<D : std::fmt::Display>(mut self, title : D) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, size : Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_scale_policy(mut self, policy : WindowScalePolicy) -> Self {
        self.scale = policy;
        self
    }

    #[cfg(feature = "opengl")]
    pub fn with_gl_config(mut self, gl_config : Option<crate::gl::GlConfig>) -> Self {
        self.gl_config = gl_config;
        self
    }

    /// Configure mac-os specific window flags
    pub fn with_mac_os_options(mut self, f : impl FnOnce(MacOSWindowOptions) -> MacOSWindowOptions) -> Self {
        self.mac_os_options = Some(f(self.mac_os_options.unwrap_or_default()));
        self
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn style_mask(&self) -> cocoa::appkit::NSWindowStyleMask {
        cocoa::appkit::NSWindowStyleMask::from_bits_truncate(self.mac_os_options.unwrap_or_default().raw_value())
    }
}

/// Represents window style flags specific to Mac OS.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MacOSWindowOptions {
    value : u64,
}

impl Default for MacOSWindowOptions {
    fn default() -> Self {
        Self { value: Default::default() }
            .titled(true)
            .closable(true)
            .miniaturizable(true)
    }
}

impl MacOSWindowOptions {

    fn raw_value(&self) -> u64 {
        self.value
    }

    /// Removes all default or currently present style flags.
    pub fn borderless(mut self) -> Self {
        self.value = 0;
        self
    }

    /// Window has a close button
    pub fn closable(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 0;
        } else {
            self.value &= !(1 << 0)
        }
        self
    }

    /// Set the window title
    pub fn titled(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 1;
        } else {
            self.value &= !(1 << 1);
        }
        self
    }

    /// Window can be minimized
    pub fn miniaturizable(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 2;
        } else {
            self.value &= !(1 << 2);
        }
        self
    }

    /// Window can be resized
    pub fn resizable(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 3;
        } else {
            self.value &= !(1 << 3);
        }
        self
    }

    /// Window can be dragged by its background
    pub fn textured(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 8;
        } else {
            self.value &= !(1 << 8);
        }
        self
    }

    /// The window paints into its title area
    pub fn unified_titlebar(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 12;
        } else {
            self.value &= !(1 << 12);
        }
        self
    }

    /// Create a full screen window
    pub fn full_screen(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 14;
        } else {
            self.value &= !(1 << 14);
        }
        self
    }

    pub fn full_size(mut self, value : bool) -> Self {
        if value {
            self.value |= 1 << 15;
        } else {
            self.value &= !(1 << 15);
        }
        self
    }
}
