#[doc = "Register `P3IES` reader"]
pub struct R(crate::R<P3IES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3IES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3IES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3IES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3IES` writer"]
pub struct W(crate::W<P3IES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3IES_SPEC>;
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
impl From<crate::W<P3IES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P3IES_SPEC>) -> Self {
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
#[doc = "Port 3 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ies](index.html) module"]
pub struct P3IES_SPEC;
impl crate::RegisterSpec for P3IES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3ies::R](R) reader structure"]
impl crate::Readable for P3IES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3ies::W](W) writer structure"]
impl crate::Writable for P3IES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3IES to value 0"]
impl crate::Resettable for P3IES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
