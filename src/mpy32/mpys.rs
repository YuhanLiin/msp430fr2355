#[doc = "Register `MPYS` reader"]
pub type R = crate::R<MpysSpec>;
#[doc = "Register `MPYS` writer"]
pub type W = crate::W<MpysSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16-bit operand one signed multiply\n\nYou can [`read`](crate::Reg::read) this register and get [`mpys::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpys::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpysSpec;
impl crate::RegisterSpec for MpysSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpys::R`](R) reader structure"]
impl crate::Readable for MpysSpec {}
#[doc = "`write(|w| ..)` method takes [`mpys::W`](W) writer structure"]
impl crate::Writable for MpysSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPYS to value 0"]
impl crate::Resettable for MpysSpec {}
