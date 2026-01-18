#[doc = "Register `RESHI` reader"]
pub type R = crate::R<ReshiSpec>;
#[doc = "Register `RESHI` writer"]
pub type W = crate::W<ReshiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16x16-bit result high word\n\nYou can [`read`](crate::Reg::read) this register and get [`reshi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reshi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReshiSpec;
impl crate::RegisterSpec for ReshiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reshi::R`](R) reader structure"]
impl crate::Readable for ReshiSpec {}
#[doc = "`write(|w| ..)` method takes [`reshi::W`](W) writer structure"]
impl crate::Writable for ReshiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESHI to value 0"]
impl crate::Resettable for ReshiSpec {}
