#[doc = "Register `BAKMEM0` reader"]
pub type R = crate::R<Bakmem0Spec>;
#[doc = "Register `BAKMEM0` writer"]
pub type W = crate::W<Bakmem0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory registers. Backup Memory 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem0Spec;
impl crate::RegisterSpec for Bakmem0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem0::R`](R) reader structure"]
impl crate::Readable for Bakmem0Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem0::W`](W) writer structure"]
impl crate::Writable for Bakmem0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM0 to value 0"]
impl crate::Resettable for Bakmem0Spec {}
