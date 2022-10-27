#[doc = "Register `SAC2DAT` reader"]
pub struct R(crate::R<SAC2DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAC2DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAC2DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAC2DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAC2DAT` writer"]
pub struct W(crate::W<SAC2DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAC2DAT_SPEC>;
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
impl From<crate::W<SAC2DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAC2DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACData` reader - SAC DAC data in unsigned format."]
pub type DACDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACData` writer - SAC DAC data in unsigned format."]
pub type DACDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SAC2DAT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - SAC DAC data in unsigned format."]
    #[inline(always)]
    pub fn dacdata(&self) -> DACDATA_R {
        DACDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - SAC DAC data in unsigned format."]
    #[inline(always)]
    pub fn dacdata(&mut self) -> DACDATA_W<0> {
        DACDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAC DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2dat](index.html) module"]
pub struct SAC2DAT_SPEC;
impl crate::RegisterSpec for SAC2DAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sac2dat::R](R) reader structure"]
impl crate::Readable for SAC2DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sac2dat::W](W) writer structure"]
impl crate::Writable for SAC2DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAC2DAT to value 0"]
impl crate::Resettable for SAC2DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
