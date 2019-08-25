#[doc = "Reader of register ICCILSR3"]
pub type R = crate::R<u16, super::ICCILSR3>;
#[doc = "Writer for register ICCILSR3"]
pub type W = crate::W<u16, super::ICCILSR3>;
#[doc = "Register ICCILSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICCILSR3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR24_A {}
impl From<ILSR24_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR24_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR24`"]
pub type ILSR24_R = crate::R<u8, ILSR24_A>;
impl ILSR24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR24_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR24`"]
pub struct ILSR24_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR24_A) -> &'a mut W {
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
pub enum ILSR25_A {}
impl From<ILSR25_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR25_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR25`"]
pub type ILSR25_R = crate::R<u8, ILSR25_A>;
impl ILSR25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR25_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR25`"]
pub struct ILSR25_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR25_A) -> &'a mut W {
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
pub enum ILSR26_A {}
impl From<ILSR26_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR26_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR26`"]
pub type ILSR26_R = crate::R<u8, ILSR26_A>;
impl ILSR26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR26_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR26`"]
pub struct ILSR26_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR26_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR27_A {}
impl From<ILSR27_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR27_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR27`"]
pub type ILSR27_R = crate::R<u8, ILSR27_A>;
impl ILSR27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR27_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR27`"]
pub struct ILSR27_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR27_A) -> &'a mut W {
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
pub enum ILSR28_A {}
impl From<ILSR28_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR28_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR28`"]
pub type ILSR28_R = crate::R<u8, ILSR28_A>;
impl ILSR28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR28_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR28`"]
pub struct ILSR28_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR28_A) -> &'a mut W {
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
pub enum ILSR29_A {}
impl From<ILSR29_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR29_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR29`"]
pub type ILSR29_R = crate::R<u8, ILSR29_A>;
impl ILSR29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR29_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR29`"]
pub struct ILSR29_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR29_A) -> &'a mut W {
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
pub enum ILSR30_A {}
impl From<ILSR30_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR30_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR30`"]
pub type ILSR30_R = crate::R<u8, ILSR30_A>;
impl ILSR30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR30_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR30`"]
pub struct ILSR30_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR30_A) -> &'a mut W {
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
pub enum ILSR31_A {}
impl From<ILSR31_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR31_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR31`"]
pub type ILSR31_R = crate::R<u8, ILSR31_A>;
impl ILSR31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR31_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR31`"]
pub struct ILSR31_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR31_A) -> &'a mut W {
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
    pub fn ilsr24(&self) -> ILSR24_R {
        ILSR24_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr25(&self) -> ILSR25_R {
        ILSR25_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr26(&self) -> ILSR26_R {
        ILSR26_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr27(&self) -> ILSR27_R {
        ILSR27_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr28(&self) -> ILSR28_R {
        ILSR28_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr29(&self) -> ILSR29_R {
        ILSR29_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr30(&self) -> ILSR30_R {
        ILSR30_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr31(&self) -> ILSR31_R {
        ILSR31_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr24(&mut self) -> ILSR24_W {
        ILSR24_W { w: self }
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr25(&mut self) -> ILSR25_W {
        ILSR25_W { w: self }
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr26(&mut self) -> ILSR26_W {
        ILSR26_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr27(&mut self) -> ILSR27_W {
        ILSR27_W { w: self }
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr28(&mut self) -> ILSR28_W {
        ILSR28_W { w: self }
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr29(&mut self) -> ILSR29_W {
        ILSR29_W { w: self }
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr30(&mut self) -> ILSR30_W {
        ILSR30_W { w: self }
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr31(&mut self) -> ILSR31_W {
        ILSR31_W { w: self }
    }
}
