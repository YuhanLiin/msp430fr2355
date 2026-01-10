#[doc = "Register `MPYS32L` reader"]
pub type R = crate::R<Mpys32lSpec>;
#[doc = "Register `MPYS32L` writer"]
pub type W = crate::W<Mpys32lSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 1 signed multiply low word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpys32l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpys32l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpys32lSpec;
impl crate::RegisterSpec for Mpys32lSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpys32l::R`](R) reader structure"]
impl crate::Readable for Mpys32lSpec {}
#[doc = "`write(|w| ..)` method takes [`mpys32l::W`](W) writer structure"]
impl crate::Writable for Mpys32lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPYS32L to value 0"]
impl crate::Resettable for Mpys32lSpec {}
