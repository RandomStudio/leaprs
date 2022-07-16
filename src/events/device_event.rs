use leap_sys::LEAP_DEVICE_EVENT;

use crate::{DeviceRef, DeviceStatus};

crate::leap_ref_struct!(DeviceEvent, LEAP_DEVICE_EVENT);

impl<'a> DeviceEvent<'a> {
    pub fn device(&self) -> DeviceRef {
        self.handle.device.into()
    }

    pub fn status(&self) -> Option<DeviceStatus> {
        DeviceStatus::from_bits(self.handle.status)
    }
}