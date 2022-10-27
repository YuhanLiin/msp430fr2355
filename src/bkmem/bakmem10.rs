#[doc = "Register `BAKMEM10` reader"]
pub struct R(crate::R<BAKMEM10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKMEM10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKMEM10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKMEM10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKMEM10` writer"]
pub struct W(crate::W<BAKMEM10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKMEM10_SPEC>;
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
impl From<crate::W<BAKMEM10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKMEM10_SPEC>) -> Self {
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
#[doc = "Backup Memory registers. Backup Memory 10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bakmem10](index.html) module"]
pub struct BAKMEM10_SPEC;
impl crate::RegisterSpec for BAKMEM10_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bakmem10::R](R) reader structure"]
impl crate::Readable for BAKMEM10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bakmem10::W](W) writer structure"]
impl crate::Writable for BAKMEM10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAKMEM10 to value 0"]
impl crate::Resettable for BAKMEM10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
