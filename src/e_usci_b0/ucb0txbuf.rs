#[doc = "Register `UCB0TXBUF` reader"]
pub struct R(crate::R<UCB0TXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0TXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0TXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0TXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0TXBUF` writer"]
pub struct W(crate::W<UCB0TXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0TXBUF_SPEC>;
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
impl From<crate::W<UCB0TXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0TXBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCTXBUF` reader - Transmit data buffer"]
pub type UCTXBUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCTXBUF` writer - Transmit data buffer"]
pub type UCTXBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCB0TXBUF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UCTXBUF_R {
        UCTXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UCTXBUF_W<0> {
        UCTXBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0txbuf](index.html) module"]
pub struct UCB0TXBUF_SPEC;
impl crate::RegisterSpec for UCB0TXBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0txbuf::R](R) reader structure"]
impl crate::Readable for UCB0TXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf::W](W) writer structure"]
impl crate::Writable for UCB0TXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0TXBUF to value 0"]
impl crate::Resettable for UCB0TXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
