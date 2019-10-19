use crate::prelude::*;
use presentation::wsi::windowsystemintegration::WindowSystemIntegration;

use std::path::Path;

pub struct WindowConfig<'a> {
    wsi: &'a WindowSystemIntegration,
}

impl<'a> WindowConfig<'a> {
    pub(crate) fn new(wsi: &'a WindowSystemIntegration) -> Self {
        WindowConfig { wsi }
    }

    pub fn set_cursor<T: AsRef<Path>>(&self, bmp: T) -> VerboseResult<()> {
        self.wsi.set_cursor(bmp)
    }

    pub fn toggle_fullscreen(&self) -> VerboseResult<()> {
        self.wsi.set_fullscreen(!self.wsi.is_fullscreen()?)
    }

    pub fn set_icon<T: AsRef<Path>>(&self, bmp: T) -> VerboseResult<()> {
        self.wsi.set_icon(bmp)
    }

    pub fn set_opacity(&self, opacity: f32) -> VerboseResult<()> {
        self.wsi.set_opacity(opacity)
    }
}
