#[doc = "Register `CRCDIRB` reader"]
pub type R = crate::R<CrcdirbSpec>;
#[doc = "Register `CRCDIRB` writer"]
pub type W = crate::W<CrcdirbSpec>;
#[doc = "Field `CRCDIRB` reader - CRC data in, bits reversed"]
pub type CrcdirbR = crate::FieldReader<u16>;
#[doc = "Field `CRCDIRB` writer - CRC data in, bits reversed"]
pub type CrcdirbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - CRC data in, bits reversed"]
    #[inline(always)]
    pub fn crcdirb(&self) -> CrcdirbR {
        CrcdirbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC data in, bits reversed"]
    #[inline(always)]
    pub fn crcdirb(&mut self) -> CrcdirbW<'_, CrcdirbSpec> {
        CrcdirbW::new(self, 0)
    }
}
#[doc = "CRC Data In Reverse Byte\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdirb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdirb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdirbSpec;
impl crate::RegisterSpec for CrcdirbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcdirb::R`](R) reader structure"]
impl crate::Readable for CrcdirbSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdirb::W`](W) writer structure"]
impl crate::Writable for CrcdirbSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCDIRB to value 0"]
impl crate::Resettable for CrcdirbSpec {}
