#[doc = "Register `BAKMEM13` reader"]
pub struct R(crate::R<BAKMEM13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKMEM13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKMEM13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKMEM13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKMEM13` writer"]
pub struct W(crate::W<BAKMEM13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKMEM13_SPEC>;
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
impl From<crate::W<BAKMEM13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKMEM13_SPEC>) -> Self {
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
#[doc = "Backup Memory 13.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bakmem13](index.html) module"]
pub struct BAKMEM13_SPEC;
impl crate::RegisterSpec for BAKMEM13_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bakmem13::R](R) reader structure"]
impl crate::Readable for BAKMEM13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bakmem13::W](W) writer structure"]
impl crate::Writable for BAKMEM13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAKMEM13 to value 0"]
impl crate::Resettable for BAKMEM13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
