#[doc = "Register `REF_CAL_LENGTH` reader"]
pub type R = crate::R<RefCalLengthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "REF calibration length\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cal_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefCalLengthSpec;
impl crate::RegisterSpec for RefCalLengthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ref_cal_length::R`](R) reader structure"]
impl crate::Readable for RefCalLengthSpec {}
#[doc = "`reset()` method sets REF_CAL_LENGTH to value 0"]
impl crate::Resettable for RefCalLengthSpec {}
