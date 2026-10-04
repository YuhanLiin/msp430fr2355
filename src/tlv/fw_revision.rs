#[doc = "Register `FW_REVISION` reader"]
pub type R = crate::R<FwRevisionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Firmware revision\n\nYou can [`read`](crate::Reg::read) this register and get [`fw_revision::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRevisionSpec;
impl crate::RegisterSpec for FwRevisionSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fw_revision::R`](R) reader structure"]
impl crate::Readable for FwRevisionSpec {}
#[doc = "`reset()` method sets FW_REVISION to value 0"]
impl crate::Resettable for FwRevisionSpec {}
