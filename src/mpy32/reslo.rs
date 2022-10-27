#[doc = "Register `RESLO` reader"]
pub struct R(crate::R<RESLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESLO` writer"]
pub struct W(crate::W<RESLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESLO_SPEC>;
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
impl From<crate::W<RESLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESLO_SPEC>) -> Self {
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
#[doc = "16x16-bit result low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reslo](index.html) module"]
pub struct RESLO_SPEC;
impl crate::RegisterSpec for RESLO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [reslo::R](R) reader structure"]
impl crate::Readable for RESLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reslo::W](W) writer structure"]
impl crate::Writable for RESLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESLO to value 0"]
impl crate::Resettable for RESLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
