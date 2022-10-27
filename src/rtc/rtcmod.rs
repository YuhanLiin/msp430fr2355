#[doc = "Register `RTCMOD` reader"]
pub struct R(crate::R<RTCMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCMOD` writer"]
pub struct W(crate::W<RTCMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCMOD_SPEC>;
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
impl From<crate::W<RTCMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCMOD_SPEC>) -> Self {
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
#[doc = "RTC Counter Modulo Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcmod](index.html) module"]
pub struct RTCMOD_SPEC;
impl crate::RegisterSpec for RTCMOD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcmod::R](R) reader structure"]
impl crate::Readable for RTCMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcmod::W](W) writer structure"]
impl crate::Writable for RTCMOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCMOD to value 0"]
impl crate::Resettable for RTCMOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
