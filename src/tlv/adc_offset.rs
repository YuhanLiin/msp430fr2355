#[doc = "Register `ADC_OFFSET` reader"]
pub type R = crate::R<AdcOffsetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ADC offset\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_offset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcOffsetSpec;
impl crate::RegisterSpec for AdcOffsetSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc_offset::R`](R) reader structure"]
impl crate::Readable for AdcOffsetSpec {}
#[doc = "`reset()` method sets ADC_OFFSET to value 0"]
impl crate::Resettable for AdcOffsetSpec {}
