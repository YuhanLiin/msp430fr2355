#[doc = "Reader of register ICCILSR2"]
pub type R = crate::R<u16, super::ICCILSR2>;
#[doc = "Writer for register ICCILSR2"]
pub type W = crate::W<u16, super::ICCILSR2>;
#[doc = "Register ICCILSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICCILSR2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ILSR16`"]
pub type ILSR16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR16`"]
pub struct ILSR16_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ILSR17`"]
pub type ILSR17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR17`"]
pub struct ILSR17_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ILSR18`"]
pub type ILSR18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR18`"]
pub struct ILSR18_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ILSR19`"]
pub type ILSR19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR19`"]
pub struct ILSR19_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `ILSR20`"]
pub type ILSR20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR20`"]
pub struct ILSR20_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ILSR21`"]
pub type ILSR21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR21`"]
pub struct ILSR21_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `ILSR22`"]
pub type ILSR22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR22`"]
pub struct ILSR22_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ILSR23`"]
pub type ILSR23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILSR23`"]
pub struct ILSR23_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr16(&self) -> ILSR16_R {
        ILSR16_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr17(&self) -> ILSR17_R {
        ILSR17_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr18(&self) -> ILSR18_R {
        ILSR18_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr19(&self) -> ILSR19_R {
        ILSR19_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr20(&self) -> ILSR20_R {
        ILSR20_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr21(&self) -> ILSR21_R {
        ILSR21_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr22(&self) -> ILSR22_R {
        ILSR22_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr23(&self) -> ILSR23_R {
        ILSR23_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr16(&mut self) -> ILSR16_W {
        ILSR16_W { w: self }
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr17(&mut self) -> ILSR17_W {
        ILSR17_W { w: self }
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr18(&mut self) -> ILSR18_W {
        ILSR18_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr19(&mut self) -> ILSR19_W {
        ILSR19_W { w: self }
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr20(&mut self) -> ILSR20_W {
        ILSR20_W { w: self }
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr21(&mut self) -> ILSR21_W {
        ILSR21_W { w: self }
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr22(&mut self) -> ILSR22_W {
        ILSR22_W { w: self }
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr23(&mut self) -> ILSR23_W {
        ILSR23_W { w: self }
    }
}
