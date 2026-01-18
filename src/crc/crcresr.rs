#[doc = "Register `CRCRESR` reader"]
pub type R = crate::R<CrcresrSpec>;
#[doc = "Register `CRCRESR` writer"]
pub type W = crate::W<CrcresrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC Result Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crcresr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcresr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcresrSpec;
impl crate::RegisterSpec for CrcresrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcresr::R`](R) reader structure"]
impl crate::Readable for CrcresrSpec {}
#[doc = "`write(|w| ..)` method takes [`crcresr::W`](W) writer structure"]
impl crate::Writable for CrcresrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCRESR to value 0"]
impl crate::Resettable for CrcresrSpec {}
