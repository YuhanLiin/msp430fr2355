#[doc = "Register `ADCLO` reader"]
pub type R = crate::R<AdcloSpec>;
#[doc = "Register `ADCLO` writer"]
pub type W = crate::W<AdcloSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adclo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adclo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcloSpec;
impl crate::RegisterSpec for AdcloSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adclo::R`](R) reader structure"]
impl crate::Readable for AdcloSpec {}
#[doc = "`write(|w| ..)` method takes [`adclo::W`](W) writer structure"]
impl crate::Writable for AdcloSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCLO to value 0"]
impl crate::Resettable for AdcloSpec {}
