#[doc = "Register `MAC32L` reader"]
pub struct R(crate::R<MAC32L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC32L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC32L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC32L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC32L` writer"]
pub struct W(crate::W<MAC32L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC32L_SPEC>;
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
impl From<crate::W<MAC32L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC32L_SPEC>) -> Self {
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
#[doc = "32-bit operand 1 multiply accumulate low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac32l](index.html) module"]
pub struct MAC32L_SPEC;
impl crate::RegisterSpec for MAC32L_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mac32l::R](R) reader structure"]
impl crate::Readable for MAC32L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac32l::W](W) writer structure"]
impl crate::Writable for MAC32L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC32L to value 0"]
impl crate::Resettable for MAC32L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
