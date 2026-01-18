#[doc = "Register `RES0` reader"]
pub type R = crate::R<Res0Spec>;
#[doc = "Register `RES0` writer"]
pub type W = crate::W<Res0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32x32-bit result 0 least significant word\n\nYou can [`read`](crate::Reg::read) this register and get [`res0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res0Spec;
impl crate::RegisterSpec for Res0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`res0::R`](R) reader structure"]
impl crate::Readable for Res0Spec {}
#[doc = "`write(|w| ..)` method takes [`res0::W`](W) writer structure"]
impl crate::Writable for Res0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RES0 to value 0"]
impl crate::Resettable for Res0Spec {}
