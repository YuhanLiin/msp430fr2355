#[doc = "Reader of register SAC0DAT"]
pub type R = crate::R<u16, super::SAC0DAT>;
#[doc = "Writer for register SAC0DAT"]
pub type W = crate::W<u16, super::SAC0DAT>;
#[doc = "Register SAC0DAT `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC0DAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACData`"]
pub type DACDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DACData`"]
pub struct DACDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DACDATA_W<'a> {
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
