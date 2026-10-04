#[doc = "Register `BAKMEM6` reader"]
pub type R = crate::R<Bakmem6Spec>;
#[doc = "Register `BAKMEM6` writer"]
pub type W = crate::W<Bakmem6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem6Spec;
impl crate::RegisterSpec for Bakmem6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem6::R`](R) reader structure"]
impl crate::Readable for Bakmem6Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem6::W`](W) writer structure"]
impl crate::Writable for Bakmem6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM6 to value 0"]
impl crate::Resettable for Bakmem6Spec {}
