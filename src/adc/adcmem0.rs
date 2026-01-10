#[doc = "Register `ADCMEM0` reader"]
pub type R = crate::R<Adcmem0Spec>;
#[doc = "Register `ADCMEM0` writer"]
pub type W = crate::W<Adcmem0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Conversion Memory Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmem0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmem0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmem0Spec;
impl crate::RegisterSpec for Adcmem0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmem0::R`](R) reader structure"]
impl crate::Readable for Adcmem0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmem0::W`](W) writer structure"]
impl crate::Writable for Adcmem0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMEM0 to value 0"]
impl crate::Resettable for Adcmem0Spec {}
