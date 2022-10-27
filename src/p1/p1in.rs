#[doc = "Register `P1IN` reader"]
pub struct R(crate::R<P1IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IN` writer"]
pub struct W(crate::W<P1IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IN_SPEC>;
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
impl From<crate::W<P1IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IN_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1in](index.html) module"]
pub struct P1IN_SPEC;
impl crate::RegisterSpec for P1IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1in::R](R) reader structure"]
impl crate::Readable for P1IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1in::W](W) writer structure"]
impl crate::Writable for P1IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IN to value 0"]
impl crate::Resettable for P1IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
