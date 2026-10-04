#[doc = "Register `PMMCTL1` reader"]
pub type R = crate::R<Pmmctl1Spec>;
#[doc = "Register `PMMCTL1` writer"]
pub type W = crate::W<Pmmctl1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Power Management Module Control Register 1. Allows manual overwrite of predictive LDO settings.\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl1Spec;
impl crate::RegisterSpec for Pmmctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl1::R`](R) reader structure"]
impl crate::Readable for Pmmctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl1::W`](W) writer structure"]
impl crate::Writable for Pmmctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL1 to value 0"]
impl crate::Resettable for Pmmctl1Spec {}
