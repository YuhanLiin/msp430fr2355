#[doc = "Register `MACS32H` reader"]
pub type R = crate::R<Macs32hSpec>;
#[doc = "Register `MACS32H` writer"]
pub type W = crate::W<Macs32hSpec>;
#[doc = "Field `MACS32H` reader - 32-bit operand 1 signed multiply accumulate high word"]
pub type Macs32hR = crate::FieldReader;
#[doc = "Field `MACS32H` writer - 32-bit operand 1 signed multiply accumulate high word"]
pub type Macs32hW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 32-bit operand 1 signed multiply accumulate high word"]
    #[inline(always)]
    pub fn macs32h(&self) -> Macs32hR {
        Macs32hR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 32-bit operand 1 signed multiply accumulate high word"]
    #[inline(always)]
    pub fn macs32h(&mut self) -> Macs32hW<'_, Macs32hSpec> {
        Macs32hW::new(self, 8)
    }
}
#[doc = "32-bit operand 1 signed multiply accumulate high word\n\nYou can [`read`](crate::Reg::read) this register and get [`macs32h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macs32h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macs32hSpec;
impl crate::RegisterSpec for Macs32hSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`macs32h::R`](R) reader structure"]
impl crate::Readable for Macs32hSpec {}
#[doc = "`write(|w| ..)` method takes [`macs32h::W`](W) writer structure"]
impl crate::Writable for Macs32hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACS32H to value 0"]
impl crate::Resettable for Macs32hSpec {}
