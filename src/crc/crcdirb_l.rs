#[doc = "Register `CRCDIRB_L` reader"]
pub type R = crate::R<CrcdirbLSpec>;
#[doc = "Register `CRCDIRB_L` writer"]
pub type W = crate::W<CrcdirbLSpec>;
#[doc = "Field `CRCDIRB` reader - CRC data in, one byte, bits reversed"]
pub type CrcdirbR = crate::FieldReader;
#[doc = "Field `CRCDIRB` writer - CRC data in, one byte, bits reversed"]
pub type CrcdirbW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - CRC data in, one byte, bits reversed"]
    #[inline(always)]
    pub fn crcdirb(&self) -> CrcdirbR {
        CrcdirbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC data in, one byte, bits reversed"]
    #[inline(always)]
    pub fn crcdirb(&mut self) -> CrcdirbW<'_, CrcdirbLSpec> {
        CrcdirbW::new(self, 0)
    }
}
#[doc = "CRC Data In Reverse Byte Register, lower byte\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdirb_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdirb_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdirbLSpec;
impl crate::RegisterSpec for CrcdirbLSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcdirb_l::R`](R) reader structure"]
impl crate::Readable for CrcdirbLSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdirb_l::W`](W) writer structure"]
impl crate::Writable for CrcdirbLSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCDIRB_L to value 0"]
impl crate::Resettable for CrcdirbLSpec {}
