#[doc = "Register `RES3` reader"]
pub type R = crate::R<Res3Spec>;
#[doc = "Register `RES3` writer"]
pub type W = crate::W<Res3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32x32-bit result 3 most significant word\n\nYou can [`read`](crate::Reg::read) this register and get [`res3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res3Spec;
impl crate::RegisterSpec for Res3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`res3::R`](R) reader structure"]
impl crate::Readable for Res3Spec {}
#[doc = "`write(|w| ..)` method takes [`res3::W`](W) writer structure"]
impl crate::Writable for Res3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RES3 to value 0"]
impl crate::Resettable for Res3Spec {}
