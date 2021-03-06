#[doc = "Reader of register SAC2DACSTS"]
pub type R = crate::R<u16, super::SAC2DACSTS>;
#[doc = "Writer for register SAC2DACSTS"]
pub type W = crate::W<u16, super::SAC2DACSTS>;
#[doc = "Register SAC2DACSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC2DACSTS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACIFG`"]
pub type DACIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACIFG`"]
pub struct DACIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DACIFG_W<'a> {
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
