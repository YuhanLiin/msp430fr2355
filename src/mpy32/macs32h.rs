#[doc = "Register `MACS32H` reader"]
pub struct R(crate::R<MACS32H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACS32H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACS32H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACS32H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACS32H` writer"]
pub struct W(crate::W<MACS32H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACS32H_SPEC>;
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
impl From<crate::W<MACS32H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACS32H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACS32H` reader - 32-bit operand 1 signed multiply accumulate high word"]
pub type MACS32H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MACS32H` writer - 32-bit operand 1 signed multiply accumulate high word"]
pub type MACS32H_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MACS32H_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - 32-bit operand 1 signed multiply accumulate high word"]
    #[inline(always)]
    pub fn macs32h(&self) -> MACS32H_R {
        MACS32H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 32-bit operand 1 signed multiply accumulate high word"]
    #[inline(always)]
    pub fn macs32h(&mut self) -> MACS32H_W<8> {
        MACS32H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32-bit operand 1 signed multiply accumulate high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macs32h](index.html) module"]
pub struct MACS32H_SPEC;
impl crate::RegisterSpec for MACS32H_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [macs32h::R](R) reader structure"]
impl crate::Readable for MACS32H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macs32h::W](W) writer structure"]
impl crate::Writable for MACS32H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACS32H to value 0"]
impl crate::Resettable for MACS32H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
