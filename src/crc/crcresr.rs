#[doc = "Register `CRCRESR` reader"]
pub struct R(crate::R<CRCRESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCRESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCRESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCRESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCRESR` writer"]
pub struct W(crate::W<CRCRESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCRESR_SPEC>;
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
impl From<crate::W<CRCRESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCRESR_SPEC>) -> Self {
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
#[doc = "CRC Result Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcresr](index.html) module"]
pub struct CRCRESR_SPEC;
impl crate::RegisterSpec for CRCRESR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcresr::R](R) reader structure"]
impl crate::Readable for CRCRESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcresr::W](W) writer structure"]
impl crate::Writable for CRCRESR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCRESR to value 0"]
impl crate::Resettable for CRCRESR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
