#[doc = "Register `BAKMEM5` reader"]
pub struct R(crate::R<BAKMEM5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKMEM5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKMEM5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKMEM5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKMEM5` writer"]
pub struct W(crate::W<BAKMEM5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKMEM5_SPEC>;
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
impl From<crate::W<BAKMEM5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKMEM5_SPEC>) -> Self {
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
#[doc = "Backup Memory 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bakmem5](index.html) module"]
pub struct BAKMEM5_SPEC;
impl crate::RegisterSpec for BAKMEM5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bakmem5::R](R) reader structure"]
impl crate::Readable for BAKMEM5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bakmem5::W](W) writer structure"]
impl crate::Writable for BAKMEM5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAKMEM5 to value 0"]
impl crate::Resettable for BAKMEM5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
