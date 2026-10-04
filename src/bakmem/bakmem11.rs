#[doc = "Register `BAKMEM11` reader"]
pub type R = crate::R<Bakmem11Spec>;
#[doc = "Register `BAKMEM11` writer"]
pub type W = crate::W<Bakmem11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem11Spec;
impl crate::RegisterSpec for Bakmem11Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem11::R`](R) reader structure"]
impl crate::Readable for Bakmem11Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem11::W`](W) writer structure"]
impl crate::Writable for Bakmem11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM11 to value 0"]
impl crate::Resettable for Bakmem11Spec {}
