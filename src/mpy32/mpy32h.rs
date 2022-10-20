#[doc = "Register `MPY32H` reader"]
pub struct R(crate::R<MPY32H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPY32H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPY32H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPY32H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPY32H` writer"]
pub struct W(crate::W<MPY32H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPY32H_SPEC>;
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
impl From<crate::W<MPY32H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPY32H_SPEC>) -> Self {
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
#[doc = "32-bit operand 1 multiply high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32h](index.html) module"]
pub struct MPY32H_SPEC;
impl crate::RegisterSpec for MPY32H_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpy32h::R](R) reader structure"]
impl crate::Readable for MPY32H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpy32h::W](W) writer structure"]
impl crate::Writable for MPY32H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPY32H to value 0"]
impl crate::Resettable for MPY32H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
