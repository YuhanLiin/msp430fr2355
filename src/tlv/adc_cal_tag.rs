#[doc = "Register `ADC_CAL_TAG` reader"]
pub type R = crate::R<AdcCalTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ADC calibration tag\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cal_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCalTagSpec;
impl crate::RegisterSpec for AdcCalTagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc_cal_tag::R`](R) reader structure"]
impl crate::Readable for AdcCalTagSpec {}
#[doc = "`reset()` method sets ADC_CAL_TAG to value 0"]
impl crate::Resettable for AdcCalTagSpec {}
