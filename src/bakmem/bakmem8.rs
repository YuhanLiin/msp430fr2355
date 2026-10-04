#[doc = "Register `BAKMEM8` reader"]
pub type R = crate::R<Bakmem8Spec>;
#[doc = "Register `BAKMEM8` writer"]
pub type W = crate::W<Bakmem8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Memory 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`bakmem8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bakmem8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bakmem8Spec;
impl crate::RegisterSpec for Bakmem8Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem8::R`](R) reader structure"]
impl crate::Readable for Bakmem8Spec {}
#[doc = "`write(|w| ..)` method takes [`bakmem8::W`](W) writer structure"]
impl crate::Writable for Bakmem8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAKMEM8 to value 0"]
impl crate::Resettable for Bakmem8Spec {}
