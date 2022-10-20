#[doc = "Register `MACS` reader"]
pub struct R(crate::R<MACS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACS` writer"]
pub struct W(crate::W<MACS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACS_SPEC>;
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
impl From<crate::W<MACS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACS_SPEC>) -> Self {
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
#[doc = "16-bit operand one signed multiply accumulate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macs](index.html) module"]
pub struct MACS_SPEC;
impl crate::RegisterSpec for MACS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [macs::R](R) reader structure"]
impl crate::Readable for MACS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macs::W](W) writer structure"]
impl crate::Writable for MACS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACS to value 0"]
impl crate::Resettable for MACS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
