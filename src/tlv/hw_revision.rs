#[doc = "Register `HW_REVISION` reader"]
pub type R = crate::R<HwRevisionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Hardware revision\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_revision::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwRevisionSpec;
impl crate::RegisterSpec for HwRevisionSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hw_revision::R`](R) reader structure"]
impl crate::Readable for HwRevisionSpec {}
#[doc = "`reset()` method sets HW_REVISION to value 0"]
impl crate::Resettable for HwRevisionSpec {}
