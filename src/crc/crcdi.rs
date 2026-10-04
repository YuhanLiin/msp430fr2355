#[doc = "Register `CRCDI` reader"]
pub type R = crate::R<CrcdiSpec>;
#[doc = "Register `CRCDI` writer"]
pub type W = crate::W<CrcdiSpec>;
#[doc = "Field `CRCDI` reader - CRC data in"]
pub type CrcdiR = crate::FieldReader<u16>;
#[doc = "Field `CRCDI` writer - CRC data in"]
pub type CrcdiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - CRC data in"]
    #[inline(always)]
    pub fn crcdi(&self) -> CrcdiR {
        CrcdiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC data in"]
    #[inline(always)]
    pub fn crcdi(&mut self) -> CrcdiW<'_, CrcdiSpec> {
        CrcdiW::new(self, 0)
    }
}
#[doc = "CRC Data In\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdiSpec;
impl crate::RegisterSpec for CrcdiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcdi::R`](R) reader structure"]
impl crate::Readable for CrcdiSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdi::W`](W) writer structure"]
impl crate::Writable for CrcdiSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCDI to value 0"]
impl crate::Resettable for CrcdiSpec {}
