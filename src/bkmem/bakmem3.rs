#[doc = "Register `BAKMEM3` reader"]
pub type R = crate::R<Bakmem3Spec>;
#[doc = "Register `BAKMEM3` writer"]
pub type W = crate::W<Bakmem3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem3Spec;
impl crate::RegisterSpec for Bakmem3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem3::R`](R) reader structure"]
impl crate::Readable for Bakmem3Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem3::W`](W) writer structure"]
impl crate::Writable for Bakmem3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM3 to value 0"]
impl crate::Resettable for Bakmem3Spec {}
