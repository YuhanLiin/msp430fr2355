#[doc = "Register `SYSJMBI0` reader"]
pub type R = crate::R<Sysjmbi0Spec>;
#[doc = "Register `SYSJMBI0` writer"]
pub type W = crate::W<Sysjmbi0Spec>;
#[doc = "Field `MSGLO` reader - JTAG mailbox incoming message low byte"]
pub type MsgloR = crate::FieldReader;
#[doc = "Field `MSGLO` writer - JTAG mailbox incoming message low byte"]
pub type MsgloW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MSGHI` reader - JTAG mailbox incoming message high byte"]
pub type MsghiR = crate::FieldReader;
#[doc = "Field `MSGHI` writer - JTAG mailbox incoming message high byte"]
pub type MsghiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - JTAG mailbox incoming message low byte"]
    #[inline(always)]
    pub fn msglo(&self) -> MsgloR {
        MsgloR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - JTAG mailbox incoming message high byte"]
    #[inline(always)]
    pub fn msghi(&self) -> MsghiR {
        MsghiR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - JTAG mailbox incoming message low byte"]
    #[inline(always)]
    pub fn msglo(&mut self) -> MsgloW<'_, Sysjmbi0Spec> {
        MsgloW::new(self, 0)
    }
    #[doc = "Bits 8:15 - JTAG mailbox incoming message high byte"]
    #[inline(always)]
    pub fn msghi(&mut self) -> MsghiW<'_, Sysjmbi0Spec> {
        MsghiW::new(self, 8)
    }
}
#[doc = "JTAG Mailbox Input\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbi0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbi0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sysjmbi0Spec;
impl crate::RegisterSpec for Sysjmbi0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbi0::R`](R) reader structure"]
impl crate::Readable for Sysjmbi0Spec {}
#[doc = "`write(|w| ..)` method takes [`sysjmbi0::W`](W) writer structure"]
impl crate::Writable for Sysjmbi0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSJMBI0 to value 0"]
impl crate::Resettable for Sysjmbi0Spec {}
