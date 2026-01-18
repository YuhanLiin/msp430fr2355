#[doc = "Register `BAKMEM4` reader"]
pub type R = crate::R<Bakmem4Spec>;
#[doc = "Register `BAKMEM4` writer"]
pub type W = crate::W<Bakmem4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem4Spec;
impl crate::RegisterSpec for Bakmem4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem4::R`](R) reader structure"]
impl crate::Readable for Bakmem4Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem4::W`](W) writer structure"]
impl crate::Writable for Bakmem4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM4 to value 0"]
impl crate::Resettable for Bakmem4Spec {}
