#[doc = "Reader of register ICCILSR0"]
pub type R = crate::R<u16, super::ICCILSR0>;
#[doc = "Writer for register ICCILSR0"]
pub type W = crate::W<u16, super::ICCILSR0>;
#[doc = "Register ICCILSR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICCILSR0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR0_A {}
impl From<ILSR0_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR0_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR0`"]
pub type ILSR0_R = crate::R<u8, ILSR0_A>;
impl ILSR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR0_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR0`"]
pub struct ILSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR1_A {}
impl From<ILSR1_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR1_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR1`"]
pub type ILSR1_R = crate::R<u8, ILSR1_A>;
impl ILSR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR1_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR1`"]
pub struct ILSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR2_A {}
impl From<ILSR2_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR2_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR2`"]
pub type ILSR2_R = crate::R<u8, ILSR2_A>;
impl ILSR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR2_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR2`"]
pub struct ILSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR3_A {}
impl From<ILSR3_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR3_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR3`"]
pub type ILSR3_R = crate::R<u8, ILSR3_A>;
impl ILSR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR3_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR3`"]
pub struct ILSR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR4_A {}
impl From<ILSR4_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR4_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR4`"]
pub type ILSR4_R = crate::R<u8, ILSR4_A>;
impl ILSR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR4_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR4`"]
pub struct ILSR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR5_A {}
impl From<ILSR5_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR5_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR5`"]
pub type ILSR5_R = crate::R<u8, ILSR5_A>;
impl ILSR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR5_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR5`"]
pub struct ILSR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR6_A {}
impl From<ILSR6_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR6_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR6`"]
pub type ILSR6_R = crate::R<u8, ILSR6_A>;
impl ILSR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR6_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR6`"]
pub struct ILSR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILSR7_A {}
impl From<ILSR7_A> for u8 {
    #[inline(always)]
    fn from(variant: ILSR7_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ILSR7`"]
pub type ILSR7_R = crate::R<u8, ILSR7_A>;
impl ILSR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ILSR7_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ILSR7`"]
pub struct ILSR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ILSR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILSR7_A) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr0(&self) -> ILSR0_R {
        ILSR0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr1(&self) -> ILSR1_R {
        ILSR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr2(&self) -> ILSR2_R {
        ILSR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr3(&self) -> ILSR3_R {
        ILSR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr4(&self) -> ILSR4_R {
        ILSR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr5(&self) -> ILSR5_R {
        ILSR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr6(&self) -> ILSR6_R {
        ILSR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr7(&self) -> ILSR7_R {
        ILSR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr0(&mut self) -> ILSR0_W {
        ILSR0_W { w: self }
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr1(&mut self) -> ILSR1_W {
        ILSR1_W { w: self }
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr2(&mut self) -> ILSR2_W {
        ILSR2_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr3(&mut self) -> ILSR3_W {
        ILSR3_W { w: self }
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr4(&mut self) -> ILSR4_W {
        ILSR4_W { w: self }
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr5(&mut self) -> ILSR5_W {
        ILSR5_W { w: self }
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr6(&mut self) -> ILSR6_W {
        ILSR6_W { w: self }
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr7(&mut self) -> ILSR7_W {
        ILSR7_W { w: self }
    }
}
