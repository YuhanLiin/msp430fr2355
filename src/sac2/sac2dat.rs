#[doc = "Reader of register SAC2DAT"]
pub type R = crate::R<u16, super::SAC2DAT>;
#[doc = "Writer for register SAC2DAT"]
pub type W = crate::W<u16, super::SAC2DAT>;
#[doc = "Register SAC2DAT `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC2DAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAC DAC data in unsigned format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACDATA_A {}
impl From<DACDATA_A> for u16 {
    #[inline(always)]
    fn from(variant: DACDATA_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `DACData`"]
pub type DACDATA_R = crate::R<u16, DACDATA_A>;
impl DACDATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, DACDATA_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `DACData`"]
pub struct DACDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DACDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACDATA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u16) & 0x0fff);
        self.w
    }
}
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
    pub fn dacdata(&mut self) -> DACDATA_W {
        DACDATA_W { w: self }
    }
}
