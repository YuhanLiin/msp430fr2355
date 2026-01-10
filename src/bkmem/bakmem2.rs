#[doc = "Register `BAKMEM2` reader"]
pub type R = crate::R<Bakmem2Spec>;
#[doc = "Register `BAKMEM2` writer"]
pub type W = crate::W<Bakmem2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem2Spec;
impl crate::RegisterSpec for Bakmem2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem2::R`](R) reader structure"]
impl crate::Readable for Bakmem2Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem2::W`](W) writer structure"]
impl crate::Writable for Bakmem2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM2 to value 0"]
impl crate::Resettable for Bakmem2Spec {}
