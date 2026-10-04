#[doc = "Register `BAKMEM7` reader"]
pub type R = crate::R<Bakmem7Spec>;
#[doc = "Register `BAKMEM7` writer"]
pub type W = crate::W<Bakmem7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem7Spec;
impl crate::RegisterSpec for Bakmem7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem7::R`](R) reader structure"]
impl crate::Readable for Bakmem7Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem7::W`](W) writer structure"]
impl crate::Writable for Bakmem7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM7 to value 0"]
impl crate::Resettable for Bakmem7Spec {}
