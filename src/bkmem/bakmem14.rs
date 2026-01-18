#[doc = "Register `BAKMEM14` reader"]
pub type R = crate::R<Bakmem14Spec>;
#[doc = "Register `BAKMEM14` writer"]
pub type W = crate::W<Bakmem14Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem14Spec;
impl crate::RegisterSpec for Bakmem14Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem14::R`](R) reader structure"]
impl crate::Readable for Bakmem14Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem14::W`](W) writer structure"]
impl crate::Writable for Bakmem14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM14 to value 0"]
impl crate::Resettable for Bakmem14Spec {}
