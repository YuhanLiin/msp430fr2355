#[doc = "Register `BAKMEM5` reader"]
pub type R = crate::R<Bakmem5Spec>;
#[doc = "Register `BAKMEM5` writer"]
pub type W = crate::W<Bakmem5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem5Spec;
impl crate::RegisterSpec for Bakmem5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem5::R`](R) reader structure"]
impl crate::Readable for Bakmem5Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem5::W`](W) writer structure"]
impl crate::Writable for Bakmem5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM5 to value 0"]
impl crate::Resettable for Bakmem5Spec {}
