#[doc = "Register `TB3CCR0` reader"]
pub type R = crate::R<Tb3ccr0Spec>;
#[doc = "Register `TB3CCR0` writer"]
pub type W = crate::W<Tb3ccr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb3ccr0Spec;
impl crate::RegisterSpec for Tb3ccr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb3ccr0::R`](R) reader structure"]
impl crate::Readable for Tb3ccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`tb3ccr0::W`](W) writer structure"]
impl crate::Writable for Tb3ccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB3CCR0 to value 0"]
impl crate::Resettable for Tb3ccr0Spec {}
