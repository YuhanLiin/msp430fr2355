#[doc = "Register `MACS32L` reader"]
pub type R = crate::R<Macs32lSpec>;
#[doc = "Register `MACS32L` writer"]
pub type W = crate::W<Macs32lSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 1 signed multiply accumulate low word\n\nYou can [`read`](crate::Reg::read) this register and get [`macs32l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macs32l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macs32lSpec;
impl crate::RegisterSpec for Macs32lSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`macs32l::R`](R) reader structure"]
impl crate::Readable for Macs32lSpec {}
#[doc = "`write(|w| ..)` method takes [`macs32l::W`](W) writer structure"]
impl crate::Writable for Macs32lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACS32L to value 0"]
impl crate::Resettable for Macs32lSpec {}
