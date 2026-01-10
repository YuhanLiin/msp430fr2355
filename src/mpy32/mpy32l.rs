#[doc = "Register `MPY32L` reader"]
pub type R = crate::R<Mpy32lSpec>;
#[doc = "Register `MPY32L` writer"]
pub type W = crate::W<Mpy32lSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 1 multiply low word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy32l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy32l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpy32lSpec;
impl crate::RegisterSpec for Mpy32lSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpy32l::R`](R) reader structure"]
impl crate::Readable for Mpy32lSpec {}
#[doc = "`write(|w| ..)` method takes [`mpy32l::W`](W) writer structure"]
impl crate::Writable for Mpy32lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPY32L to value 0"]
impl crate::Resettable for Mpy32lSpec {}
