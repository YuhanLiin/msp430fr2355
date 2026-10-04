#[doc = "Register `ADC_GAIN_FACTOR` reader"]
pub type R = crate::R<AdcGainFactorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ADC gain factor\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_gain_factor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcGainFactorSpec;
impl crate::RegisterSpec for AdcGainFactorSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc_gain_factor::R`](R) reader structure"]
impl crate::Readable for AdcGainFactorSpec {}
#[doc = "`reset()` method sets ADC_GAIN_FACTOR to value 0"]
impl crate::Resettable for AdcGainFactorSpec {}
