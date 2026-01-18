#[doc = "Register `RES2` reader"]
pub type R = crate::R<Res2Spec>;
#[doc = "Register `RES2` writer"]
pub type W = crate::W<Res2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32x32-bit result 2\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res2Spec;
impl crate::RegisterSpec for Res2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`res2::R`](R) reader structure"]
impl crate::Readable for Res2Spec {}
#[doc = "`write(|w| ..)` method takes [`res2::W`](W) writer structure"]
impl crate::Writable for Res2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RES2 to value 0"]
impl crate::Resettable for Res2Spec {}
