#[doc = "Register `CSCTL0` reader"]
pub struct R(crate::R<CSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL0` writer"]
pub struct W(crate::W<CSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL0_SPEC>;
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
impl From<crate::W<CSCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCO` reader - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
pub type DCO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCO` writer - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
pub type DCO_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL0_SPEC, u16, u16, 9, O>;
#[doc = "Field `MOD` reader - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
pub type MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOD` writer - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
pub type MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:8 - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
    #[inline(always)]
    pub fn dco(&self) -> DCO_R {
        DCO_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:13 - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
    #[inline(always)]
    pub fn dco(&mut self) -> DCO_W<0> {
        DCO_W::new(self)
    }
    #[doc = "Bits 9:13 - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W<9> {
        MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl0](index.html) module"]
pub struct CSCTL0_SPEC;
impl crate::RegisterSpec for CSCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl0::R](R) reader structure"]
impl crate::Readable for CSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl0::W](W) writer structure"]
impl crate::Writable for CSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL0 to value 0"]
impl crate::Resettable for CSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
