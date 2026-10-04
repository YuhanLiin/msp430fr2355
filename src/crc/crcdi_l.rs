#[doc = "Register `CRCDI_L` reader"]
pub type R = crate::R<CrcdiLSpec>;
#[doc = "Register `CRCDI_L` writer"]
pub type W = crate::W<CrcdiLSpec>;
#[doc = "Field `CRCDI` reader - CRC data in, one byte"]
pub type CrcdiR = crate::FieldReader;
#[doc = "Field `CRCDI` writer - CRC data in, one byte"]
pub type CrcdiW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - CRC data in, one byte"]
    #[inline(always)]
    pub fn crcdi(&self) -> CrcdiR {
        CrcdiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC data in, one byte"]
    #[inline(always)]
    pub fn crcdi(&mut self) -> CrcdiW<'_, CrcdiLSpec> {
        CrcdiW::new(self, 0)
    }
}
#[doc = "CRC Data In Register, lower byte\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdi_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdi_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdiLSpec;
impl crate::RegisterSpec for CrcdiLSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcdi_l::R`](R) reader structure"]
impl crate::Readable for CrcdiLSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdi_l::W`](W) writer structure"]
impl crate::Writable for CrcdiLSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCDI_L to value 0"]
impl crate::Resettable for CrcdiLSpec {}
