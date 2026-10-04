#[doc = "Register `BAKMEM10` reader"]
pub type R = crate::R<Bakmem10Spec>;
#[doc = "Register `BAKMEM10` writer"]
pub type W = crate::W<Bakmem10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory registers. Backup Memory 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem10Spec;
impl crate::RegisterSpec for Bakmem10Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem10::R`](R) reader structure"]
impl crate::Readable for Bakmem10Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem10::W`](W) writer structure"]
impl crate::Writable for Bakmem10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM10 to value 0"]
impl crate::Resettable for Bakmem10Spec {}
