#[doc = "Register `SAC1DACSTS` reader"]
pub struct R(crate::R<SAC1DACSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAC1DACSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAC1DACSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAC1DACSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAC1DACSTS` writer"]
pub struct W(crate::W<SAC1DACSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAC1DACSTS_SPEC>;
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
impl From<crate::W<SAC1DACSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAC1DACSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACIFG` reader - SAC DAC data update flag"]
pub type DACIFG_R = crate::BitReader<bool>;
#[doc = "Field `DACIFG` writer - SAC DAC data update flag"]
pub type DACIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC1DACSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SAC DAC data update flag"]
    #[inline(always)]
    pub fn dacifg(&self) -> DACIFG_R {
        DACIFG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAC DAC data update flag"]
    #[inline(always)]
    pub fn dacifg(&mut self) -> DACIFG_W<0> {
        DACIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1dacsts](index.html) module"]
pub struct SAC1DACSTS_SPEC;
impl crate::RegisterSpec for SAC1DACSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sac1dacsts::R](R) reader structure"]
impl crate::Readable for SAC1DACSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sac1dacsts::W](W) writer structure"]
impl crate::Writable for SAC1DACSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAC1DACSTS to value 0"]
impl crate::Resettable for SAC1DACSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
