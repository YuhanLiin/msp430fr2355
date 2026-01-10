#[doc = "Register `CRCDIRB` reader"]
pub type R = crate::R<CrcdirbSpec>;
#[doc = "Register `CRCDIRB` writer"]
pub type W = crate::W<CrcdirbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC Data In Reverse Byte\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdirb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdirb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdirbSpec;
impl crate::RegisterSpec for CrcdirbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcdirb::R`](R) reader structure"]
impl crate::Readable for CrcdirbSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdirb::W`](W) writer structure"]
impl crate::Writable for CrcdirbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCDIRB to value 0"]
impl crate::Resettable for CrcdirbSpec {}
