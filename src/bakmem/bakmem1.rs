#[doc = "Register `BAKMEM1` reader"]
pub type R = crate::R<Bakmem1Spec>;
#[doc = "Register `BAKMEM1` writer"]
pub type W = crate::W<Bakmem1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem1Spec;
impl crate::RegisterSpec for Bakmem1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem1::R`](R) reader structure"]
impl crate::Readable for Bakmem1Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem1::W`](W) writer structure"]
impl crate::Writable for Bakmem1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM1 to value 0"]
impl crate::Resettable for Bakmem1Spec {}
