#[doc = "Register `DIE_RECORD_TAG` reader"]
pub type R = crate::R<DieRecordTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die record tag\n\nYou can [`read`](crate::Reg::read) this register and get [`die_record_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieRecordTagSpec;
impl crate::RegisterSpec for DieRecordTagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`die_record_tag::R`](R) reader structure"]
impl crate::Readable for DieRecordTagSpec {}
#[doc = "`reset()` method sets DIE_RECORD_TAG to value 0"]
impl crate::Resettable for DieRecordTagSpec {}
