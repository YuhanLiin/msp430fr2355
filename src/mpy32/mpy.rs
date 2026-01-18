#[doc = "Register `MPY` reader"]
pub type R = crate::R<MpySpec>;
#[doc = "Register `MPY` writer"]
pub type W = crate::W<MpySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "16-bit operand one multiply\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpySpec;
impl crate::RegisterSpec for MpySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpy::R`](R) reader structure"]
impl crate::Readable for MpySpec {}
#[doc = "`write(|w| ..)` method takes [`mpy::W`](W) writer structure"]
impl crate::Writable for MpySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPY to value 0"]
impl crate::Resettable for MpySpec {}
