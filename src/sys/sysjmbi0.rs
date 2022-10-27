#[doc = "Register `SYSJMBI0` reader"]
pub struct R(crate::R<SYSJMBI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSJMBI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSJMBI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSJMBI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSJMBI0` writer"]
pub struct W(crate::W<SYSJMBI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSJMBI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYSJMBI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSJMBI0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSGLO` reader - JTAG mailbox incoming message low byte"]
pub type MSGLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSGLO` writer - JTAG mailbox incoming message low byte"]
pub type MSGLO_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SYSJMBI0_SPEC, u8, u8, 8, O>;
#[doc = "Field `MSGHI` reader - JTAG mailbox incoming message high byte"]
pub type MSGHI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSGHI` writer - JTAG mailbox incoming message high byte"]
pub type MSGHI_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SYSJMBI0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - JTAG mailbox incoming message low byte"]
    #[inline(always)]
    pub fn msglo(&self) -> MSGLO_R {
        MSGLO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - JTAG mailbox incoming message high byte"]
    #[inline(always)]
    pub fn msghi(&self) -> MSGHI_R {
        MSGHI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - JTAG mailbox incoming message low byte"]
    #[inline(always)]
    pub fn msglo(&mut self) -> MSGLO_W<0> {
        MSGLO_W::new(self)
    }
    #[doc = "Bits 8:15 - JTAG mailbox incoming message high byte"]
    #[inline(always)]
    pub fn msghi(&mut self) -> MSGHI_W<8> {
        MSGHI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG Mailbox Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbi0](index.html) module"]
pub struct SYSJMBI0_SPEC;
impl crate::RegisterSpec for SYSJMBI0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysjmbi0::R](R) reader structure"]
impl crate::Readable for SYSJMBI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysjmbi0::W](W) writer structure"]
impl crate::Writable for SYSJMBI0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSJMBI0 to value 0"]
impl crate::Resettable for SYSJMBI0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
