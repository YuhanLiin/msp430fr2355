#[doc = "Register `CRCDI` reader"]
pub struct R(crate::R<CRCDI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDI` writer"]
pub struct W(crate::W<CRCDI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDI_SPEC>;
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
impl From<crate::W<CRCDI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDI_SPEC>) -> Self {
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
#[doc = "CRC Data In\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdi](index.html) module"]
pub struct CRCDI_SPEC;
impl crate::RegisterSpec for CRCDI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcdi::R](R) reader structure"]
impl crate::Readable for CRCDI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdi::W](W) writer structure"]
impl crate::Writable for CRCDI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCDI to value 0"]
impl crate::Resettable for CRCDI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
