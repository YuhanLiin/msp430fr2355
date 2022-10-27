#[doc = "Register `CRCINIRES` reader"]
pub struct R(crate::R<CRCINIRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCINIRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCINIRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCINIRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCINIRES` writer"]
pub struct W(crate::W<CRCINIRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCINIRES_SPEC>;
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
impl From<crate::W<CRCINIRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCINIRES_SPEC>) -> Self {
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
#[doc = "CRC Initialization and Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcinires](index.html) module"]
pub struct CRCINIRES_SPEC;
impl crate::RegisterSpec for CRCINIRES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcinires::R](R) reader structure"]
impl crate::Readable for CRCINIRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcinires::W](W) writer structure"]
impl crate::Writable for CRCINIRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCINIRES to value 0"]
impl crate::Resettable for CRCINIRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
