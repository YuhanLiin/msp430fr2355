#[doc = "Register `SYSJMBO1` reader"]
pub struct R(crate::R<SYSJMBO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSJMBO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSJMBO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSJMBO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSJMBO1` writer"]
pub struct W(crate::W<SYSJMBO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSJMBO1_SPEC>;
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
impl From<crate::W<SYSJMBO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSJMBO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSGLO` reader - JTAG mailbox outgoing message low byte"]
pub type MSGLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSGLO` writer - JTAG mailbox outgoing message low byte"]
pub type MSGLO_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SYSJMBO1_SPEC, u8, u8, 8, O>;
#[doc = "Field `MSGHI` reader - JTAG mailbox outgoing message high byte"]
pub type MSGHI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSGHI` writer - JTAG mailbox outgoing message high byte"]
pub type MSGHI_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SYSJMBO1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - JTAG mailbox outgoing message low byte"]
    #[inline(always)]
    pub fn msglo(&self) -> MSGLO_R {
        MSGLO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - JTAG mailbox outgoing message high byte"]
    #[inline(always)]
    pub fn msghi(&self) -> MSGHI_R {
        MSGHI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - JTAG mailbox outgoing message low byte"]
    #[inline(always)]
    pub fn msglo(&mut self) -> MSGLO_W<0> {
        MSGLO_W::new(self)
    }
    #[doc = "Bits 8:15 - JTAG mailbox outgoing message high byte"]
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
#[doc = "JTAG Mailbox Output 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbo1](index.html) module"]
pub struct SYSJMBO1_SPEC;
impl crate::RegisterSpec for SYSJMBO1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysjmbo1::R](R) reader structure"]
impl crate::Readable for SYSJMBO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysjmbo1::W](W) writer structure"]
impl crate::Writable for SYSJMBO1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSJMBO1 to value 0"]
impl crate::Resettable for SYSJMBO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
