#[doc = "Reader of register ICCILSR1"]
pub type R = crate::R<u16, super::ICCILSR1>;
#[doc = "Writer for register ICCILSR1"]
pub type W = crate::W<u16, super::ICCILSR1>;
#[doc = "Register ICCILSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICCILSR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR8_A {}
impl From<ILSR8_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR8_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR8`"]
pub type ILSR8_R = crate::R<u8, ILSR8_A>;
impl ILSR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR8_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR8`"]
pub struct ILSR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR9_A {}
impl From<ILSR9_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR9_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR9`"]
pub type ILSR9_R = crate::R<u8, ILSR9_A>;
impl ILSR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR9_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR9`"]
pub struct ILSR9_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR10_A {}
impl From<ILSR10_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR10_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR10`"]
pub type ILSR10_R = crate::R<u8, ILSR10_A>;
impl ILSR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR10_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR10`"]
pub struct ILSR10_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR11_A {}
impl From<ILSR11_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR11_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR11`"]
pub type ILSR11_R = crate::R<u8, ILSR11_A>;
impl ILSR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR11_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR11`"]
pub struct ILSR11_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR12_A {}
impl From<ILSR12_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR12_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR12`"]
pub type ILSR12_R = crate::R<u8, ILSR12_A>;
impl ILSR12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR12_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR12`"]
pub struct ILSR12_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR13_A {}
impl From<ILSR13_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR13_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR13`"]
pub type ILSR13_R = crate::R<u8, ILSR13_A>;
impl ILSR13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR13_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR13`"]
pub struct ILSR13_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR14_A {}
impl From<ILSR14_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR14_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR14`"]
pub type ILSR14_R = crate::R<u8, ILSR14_A>;
impl ILSR14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR14_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR14`"]
pub struct ILSR14_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR15_A {}
impl From<ILSR15_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR15_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR15`"]
pub type ILSR15_R = crate::R<u8, ILSR15_A>;
impl ILSR15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR15_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR15`"]
pub struct ILSR15_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    pub fn ilsr8(&self) -> ILSR8_R {
        ILSR8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr9(&self) -> ILSR9_R {
        ILSR9_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr10(&self) -> ILSR10_R {
        ILSR10_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr11(&self) -> ILSR11_R {
        ILSR11_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr12(&self) -> ILSR12_R {
        ILSR12_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr13(&self) -> ILSR13_R {
        ILSR13_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr14(&self) -> ILSR14_R {
        ILSR14_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr15(&self) -> ILSR15_R {
        ILSR15_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr8(&mut self) -> ILSR8_W {
        ILSR8_W { w: self }
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr9(&mut self) -> ILSR9_W {
        ILSR9_W { w: self }
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr10(&mut self) -> ILSR10_W {
        ILSR10_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr11(&mut self) -> ILSR11_W {
        ILSR11_W { w: self }
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr12(&mut self) -> ILSR12_W {
        ILSR12_W { w: self }
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr13(&mut self) -> ILSR13_W {
        ILSR13_W { w: self }
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr14(&mut self) -> ILSR14_W {
        ILSR14_W { w: self }
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr15(&mut self) -> ILSR15_W {
        ILSR15_W { w: self }
    }
}
