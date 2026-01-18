#[doc = "Register `MPY32H` reader"]
pub type R = crate::R<Mpy32hSpec>;
#[doc = "Register `MPY32H` writer"]
pub type W = crate::W<Mpy32hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 1 multiply high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy32h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy32h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpy32hSpec;
impl crate::RegisterSpec for Mpy32hSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpy32h::R`](R) reader structure"]
impl crate::Readable for Mpy32hSpec {}
#[doc = "`write(|w| ..)` method takes [`mpy32h::W`](W) writer structure"]
impl crate::Writable for Mpy32hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPY32H to value 0"]
impl crate::Resettable for Mpy32hSpec {}
