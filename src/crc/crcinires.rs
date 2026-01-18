#[doc = "Register `CRCINIRES` reader"]
pub type R = crate::R<CrciniresSpec>;
#[doc = "Register `CRCINIRES` writer"]
pub type W = crate::W<CrciniresSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC Initialization and Result\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinires::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinires::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrciniresSpec;
impl crate::RegisterSpec for CrciniresSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcinires::R`](R) reader structure"]
impl crate::Readable for CrciniresSpec {}
#[doc = "`write(|w| ..)` method takes [`crcinires::W`](W) writer structure"]
impl crate::Writable for CrciniresSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCINIRES to value 0"]
impl crate::Resettable for CrciniresSpec {}
