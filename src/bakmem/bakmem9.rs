#[doc = "Register `BAKMEM9` reader"]
pub type R = crate::R<Bakmem9Spec>;
#[doc = "Register `BAKMEM9` writer"]
pub type W = crate::W<Bakmem9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem9Spec;
impl crate::RegisterSpec for Bakmem9Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem9::R`](R) reader structure"]
impl crate::Readable for Bakmem9Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem9::W`](W) writer structure"]
impl crate::Writable for Bakmem9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM9 to value 0"]
impl crate::Resettable for Bakmem9Spec {}
