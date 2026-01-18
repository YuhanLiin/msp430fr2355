#[doc = "Register `MAC32H` reader"]
pub type R = crate::R<Mac32hSpec>;
#[doc = "Register `MAC32H` writer"]
pub type W = crate::W<Mac32hSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "32-bit operand 1 multiply accumulate high word\n\nYou can [`read`](crate::Reg::read) this register and get [`mac32h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac32h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mac32hSpec;
impl crate::RegisterSpec for Mac32hSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mac32h::R`](R) reader structure"]
impl crate::Readable for Mac32hSpec {}
#[doc = "`write(|w| ..)` method takes [`mac32h::W`](W) writer structure"]
impl crate::Writable for Mac32hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC32H to value 0"]
impl crate::Resettable for Mac32hSpec {}
