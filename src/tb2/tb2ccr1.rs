#[doc = "Register `TB2CCR1` reader"]
pub type R = crate::R<Tb2ccr1Spec>;
#[doc = "Register `TB2CCR1` writer"]
pub type W = crate::W<Tb2ccr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb2ccr1Spec;
impl crate::RegisterSpec for Tb2ccr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb2ccr1::R`](R) reader structure"]
impl crate::Readable for Tb2ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tb2ccr1::W`](W) writer structure"]
impl crate::Writable for Tb2ccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB2CCR1 to value 0"]
impl crate::Resettable for Tb2ccr1Spec {}
