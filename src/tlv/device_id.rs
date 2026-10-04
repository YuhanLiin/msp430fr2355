#[doc = "Register `DEVICE_ID` reader"]
pub type R = crate::R<DeviceIdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceIdSpec;
impl crate::RegisterSpec for DeviceIdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`device_id::R`](R) reader structure"]
impl crate::Readable for DeviceIdSpec {}
#[doc = "`reset()` method sets DEVICE_ID to value 0"]
impl crate::Resettable for DeviceIdSpec {}
