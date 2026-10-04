#[doc = "Register `CRCINIRES` reader"]
pub type R = crate::R<CrciniresSpec>;
#[doc = "Register `CRCINIRES` writer"]
pub type W = crate::W<CrciniresSpec>;
#[doc = "Field `CRCINIRES` reader - CRC initialization (on write) and result (on read)"]
pub type CrciniresR = crate::FieldReader<u16>;
#[doc = "Field `CRCINIRES` writer - CRC initialization (on write) and result (on read)"]
pub type CrciniresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - CRC initialization (on write) and result (on read)"]
    #[inline(always)]
    pub fn crcinires(&self) -> CrciniresR {
        CrciniresR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC initialization (on write) and result (on read)"]
    #[inline(always)]
    pub fn crcinires(&mut self) -> CrciniresW<'_, CrciniresSpec> {
        CrciniresW::new(self, 0)
    }
}
#[doc = "CRC Initialization and Result\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinires::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinires::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrciniresSpec;
impl crate::RegisterSpec for CrciniresSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcinires::R`](R) reader structure"]
impl crate::Readable for CrciniresSpec {}
#[doc = "`write(|w| ..)` method takes [`crcinires::W`](W) writer structure"]
impl crate::Writable for CrciniresSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCINIRES to value 0"]
impl crate::Resettable for CrciniresSpec {}
