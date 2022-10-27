#[doc = "Register `MPYS` reader"]
pub struct R(crate::R<MPYS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPYS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPYS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPYS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPYS` writer"]
pub struct W(crate::W<MPYS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPYS_SPEC>;
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
impl From<crate::W<MPYS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPYS_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16-bit operand one signed multiply\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpys](index.html) module"]
pub struct MPYS_SPEC;
impl crate::RegisterSpec for MPYS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpys::R](R) reader structure"]
impl crate::Readable for MPYS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpys::W](W) writer structure"]
impl crate::Writable for MPYS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPYS to value 0"]
impl crate::Resettable for MPYS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
