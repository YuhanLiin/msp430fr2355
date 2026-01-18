#[doc = "Register `MPYS32H` reader"]
pub type R = crate::R<Mpys32hSpec>;
#[doc = "Register `MPYS32H` writer"]
pub type W = crate::W<Mpys32hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 1 signed multiply high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mpys32h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpys32h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpys32hSpec;
impl crate::RegisterSpec for Mpys32hSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpys32h::R`](R) reader structure"]
impl crate::Readable for Mpys32hSpec {}
#[doc = "`write(|w| ..)` method takes [`mpys32h::W`](W) writer structure"]
impl crate::Writable for Mpys32hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPYS32H to value 0"]
impl crate::Resettable for Mpys32hSpec {}
