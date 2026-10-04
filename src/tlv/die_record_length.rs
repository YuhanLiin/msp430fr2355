#[doc = "Register `DIE_RECORD_LENGTH` reader"]
pub type R = crate::R<DieRecordLengthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die record length\n\nYou can [`read`](crate::Reg::read) this register and get [`die_record_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieRecordLengthSpec;
impl crate::RegisterSpec for DieRecordLengthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`die_record_length::R`](R) reader structure"]
impl crate::Readable for DieRecordLengthSpec {}
#[doc = "`reset()` method sets DIE_RECORD_LENGTH to value 0"]
impl crate::Resettable for DieRecordLengthSpec {}
