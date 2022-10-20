#[doc = "Register `BAKMEM11` reader"]
pub struct R(crate::R<BAKMEM11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKMEM11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKMEM11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKMEM11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKMEM11` writer"]
pub struct W(crate::W<BAKMEM11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKMEM11_SPEC>;
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
impl From<crate::W<BAKMEM11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKMEM11_SPEC>) -> Self {
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
#[doc = "Backup Memory 11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bakmem11](index.html) module"]
pub struct BAKMEM11_SPEC;
impl crate::RegisterSpec for BAKMEM11_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bakmem11::R](R) reader structure"]
impl crate::Readable for BAKMEM11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bakmem11::W](W) writer structure"]
impl crate::Writable for BAKMEM11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAKMEM11 to value 0"]
impl crate::Resettable for BAKMEM11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
