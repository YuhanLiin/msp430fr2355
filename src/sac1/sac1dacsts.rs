#[doc = "Reader of register SAC1DACSTS"]
pub type R = crate::R<u16, super::SAC1DACSTS>;
#[doc = "Writer for register SAC1DACSTS"]
pub type W = crate::W<u16, super::SAC1DACSTS>;
#[doc = "Register SAC1DACSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC1DACSTS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAC DAC data update flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACIFG_A {}
impl From<DACIFG_A> for bool {
    #[inline(always)]
    fn from(variant: DACIFG_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `DACIFG`"]
pub type DACIFG_R = crate::R<bool, DACIFG_A>;
impl DACIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DACIFG_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `DACIFG`"]
pub struct DACIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DACIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SAC DAC data update flag"]
    #[inline(always)]
    pub fn dacifg(&self) -> DACIFG_R {
        DACIFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAC DAC data update flag"]
    #[inline(always)]
    pub fn dacifg(&mut self) -> DACIFG_W {
        DACIFG_W { w: self }
    }
}
