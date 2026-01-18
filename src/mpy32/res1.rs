#[doc = "Register `RES1` reader"]
pub type R = crate::R<Res1Spec>;
#[doc = "Register `RES1` writer"]
pub type W = crate::W<Res1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32x32-bit result 1\n\nYou can [`read`](crate::Reg::read) this register and get [`res1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res1Spec;
impl crate::RegisterSpec for Res1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`res1::R`](R) reader structure"]
impl crate::Readable for Res1Spec {}
#[doc = "`write(|w| ..)` method takes [`res1::W`](W) writer structure"]
impl crate::Writable for Res1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RES1 to value 0"]
impl crate::Resettable for Res1Spec {}
