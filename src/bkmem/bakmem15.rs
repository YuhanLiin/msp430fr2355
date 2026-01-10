#[doc = "Register `BAKMEM15` reader"]
pub type R = crate::R<Bakmem15Spec>;
#[doc = "Register `BAKMEM15` writer"]
pub type W = crate::W<Bakmem15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem15Spec;
impl crate::RegisterSpec for Bakmem15Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem15::R`](R) reader structure"]
impl crate::Readable for Bakmem15Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem15::W`](W) writer structure"]
impl crate::Writable for Bakmem15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM15 to value 0"]
impl crate::Resettable for Bakmem15Spec {}
