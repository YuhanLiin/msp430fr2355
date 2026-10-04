#[doc = "Register `BAKMEM12` reader"]
pub type R = crate::R<Bakmem12Spec>;
#[doc = "Register `BAKMEM12` writer"]
pub type W = crate::W<Bakmem12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem12Spec;
impl crate::RegisterSpec for Bakmem12Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem12::R`](R) reader structure"]
impl crate::Readable for Bakmem12Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem12::W`](W) writer structure"]
impl crate::Writable for Bakmem12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM12 to value 0"]
impl crate::Resettable for Bakmem12Spec {}
