#[doc = "Register `UCB0TBCNT` reader"]
pub struct R(crate::R<UCB0TBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0TBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0TBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0TBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0TBCNT` writer"]
pub struct W(crate::W<UCB0TBCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0TBCNT_SPEC>;
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
impl From<crate::W<UCB0TBCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0TBCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCTBCNT` reader - Byte counter threshold value"]
pub type UCTBCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCTBCNT` writer - Byte counter threshold value"]
pub type UCTBCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCB0TBCNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&self) -> UCTBCNT_R {
        UCTBCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&mut self) -> UCTBCNT_W<0> {
        UCTBCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0tbcnt](index.html) module"]
pub struct UCB0TBCNT_SPEC;
impl crate::RegisterSpec for UCB0TBCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0tbcnt::R](R) reader structure"]
impl crate::Readable for UCB0TBCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0tbcnt::W](W) writer structure"]
impl crate::Writable for UCB0TBCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0TBCNT to value 0"]
impl crate::Resettable for UCB0TBCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
