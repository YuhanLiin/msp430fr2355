#[doc = "Register `ADC_CAL_LENGTH` reader"]
pub type R = crate::R<AdcCalLengthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ADC calibration length\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cal_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCalLengthSpec;
impl crate::RegisterSpec for AdcCalLengthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc_cal_length::R`](R) reader structure"]
impl crate::Readable for AdcCalLengthSpec {}
#[doc = "`reset()` method sets ADC_CAL_LENGTH to value 0"]
impl crate::Resettable for AdcCalLengthSpec {}
