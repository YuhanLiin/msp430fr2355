#[doc = "Register `RTCCNT` reader"]
pub struct R(crate::R<RTCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCNT` writer"]
pub struct W(crate::W<RTCCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCNT_SPEC>;
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
impl From<crate::W<RTCCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCNT_SPEC>) -> Self {
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
#[doc = "RTC Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt](index.html) module"]
pub struct RTCCNT_SPEC;
impl crate::RegisterSpec for RTCCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtccnt::R](R) reader structure"]
impl crate::Readable for RTCCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccnt::W](W) writer structure"]
impl crate::Writable for RTCCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCNT to value 0"]
impl crate::Resettable for RTCCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
