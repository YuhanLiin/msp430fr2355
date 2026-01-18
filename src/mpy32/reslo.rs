#[doc = "Register `RESLO` reader"]
pub type R = crate::R<ResloSpec>;
#[doc = "Register `RESLO` writer"]
pub type W = crate::W<ResloSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16x16-bit result low word\n\nYou can [`read`](crate::Reg::read) this register and get [`reslo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reslo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResloSpec;
impl crate::RegisterSpec for ResloSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reslo::R`](R) reader structure"]
impl crate::Readable for ResloSpec {}
#[doc = "`write(|w| ..)` method takes [`reslo::W`](W) writer structure"]
impl crate::Writable for ResloSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESLO to value 0"]
impl crate::Resettable for ResloSpec {}
