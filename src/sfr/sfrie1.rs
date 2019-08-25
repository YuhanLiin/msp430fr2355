#[doc = "Reader of register SFRIE1"]
pub type R = crate::R<u16, super::SFRIE1>;
#[doc = "Writer for register SFRIE1"]
pub type W = crate::W<u16, super::SFRIE1>;
#[doc = "Register SFRIE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SFRIE1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Watchdog timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE,
    #[doc = "1: Interrupts enabled"]
    ENABLE,
}
impl From<WDTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WDTIE_A) -> Self {
        match variant {
            WDTIE_A::DISABLE => false,
            WDTIE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `WDTIE`"]
pub type WDTIE_R = crate::R<bool, WDTIE_A>;
impl WDTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIE_A {
        match self.bits {
            false => WDTIE_A::DISABLE,
            true => WDTIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDTIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDTIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `WDTIE`"]
pub struct WDTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDTIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDTIE_A::ENABLE)
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
#[doc = "Oscillator fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE,
    #[doc = "1: Interrupts enabled"]
    ENABLE,
}
impl From<OFIE_A> for bool {
    #[inline(always)]
    fn from(variant: OFIE_A) -> Self {
        match variant {
            OFIE_A::DISABLE => false,
            OFIE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `OFIE`"]
pub type OFIE_R = crate::R<bool, OFIE_A>;
impl OFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFIE_A {
        match self.bits {
            false => OFIE_A::DISABLE,
            true => OFIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OFIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OFIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `OFIE`"]
pub struct OFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OFIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OFIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Vacant memory access interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VMAIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE,
    #[doc = "1: Interrupts enabled"]
    ENABLE,
}
impl From<VMAIE_A> for bool {
    #[inline(always)]
    fn from(variant: VMAIE_A) -> Self {
        match variant {
            VMAIE_A::DISABLE => false,
            VMAIE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `VMAIE`"]
pub type VMAIE_R = crate::R<bool, VMAIE_A>;
impl VMAIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMAIE_A {
        match self.bits {
            false => VMAIE_A::DISABLE,
            true => VMAIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VMAIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VMAIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `VMAIE`"]
pub struct VMAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VMAIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VMAIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VMAIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "NMI pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE,
    #[doc = "1: Interrupts enabled"]
    ENABLE,
}
impl From<NMIIE_A> for bool {
    #[inline(always)]
    fn from(variant: NMIIE_A) -> Self {
        match variant {
            NMIIE_A::DISABLE => false,
            NMIIE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `NMIIE`"]
pub type NMIIE_R = crate::R<bool, NMIIE_A>;
impl NMIIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIIE_A {
        match self.bits {
            false => NMIIE_A::DISABLE,
            true => NMIIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NMIIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NMIIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `NMIIE`"]
pub struct NMIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NMIIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NMIIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "JTAG mailbox input interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBINIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE,
    #[doc = "1: Interrupts enabled"]
    ENABLE,
}
impl From<JMBINIE_A> for bool {
    #[inline(always)]
    fn from(variant: JMBINIE_A) -> Self {
        match variant {
            JMBINIE_A::DISABLE => false,
            JMBINIE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `JMBINIE`"]
pub type JMBINIE_R = crate::R<bool, JMBINIE_A>;
impl JMBINIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBINIE_A {
        match self.bits {
            false => JMBINIE_A::DISABLE,
            true => JMBINIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == JMBINIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == JMBINIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `JMBINIE`"]
pub struct JMBINIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBINIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBINIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(JMBINIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(JMBINIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "JTAG mailbox output interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBOUTIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE,
    #[doc = "1: Interrupts enabled"]
    ENABLE,
}
impl From<JMBOUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUTIE_A) -> Self {
        match variant {
            JMBOUTIE_A::DISABLE => false,
            JMBOUTIE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `JMBOUTIE`"]
pub type JMBOUTIE_R = crate::R<bool, JMBOUTIE_A>;
impl JMBOUTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUTIE_A {
        match self.bits {
            false => JMBOUTIE_A::DISABLE,
            true => JMBOUTIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == JMBOUTIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == JMBOUTIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `JMBOUTIE`"]
pub struct JMBOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBOUTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(JMBOUTIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(JMBOUTIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog timer interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VMAIE_R {
        VMAIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI pin interrupt enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JMBINIE_R {
        JMBINIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JMBOUTIE_R {
        JMBOUTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog timer interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W {
        WDTIE_W { w: self }
    }
    #[doc = "Bit 1 - Oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OFIE_W {
        OFIE_W { w: self }
    }
    #[doc = "Bit 3 - Vacant memory access interrupt enable"]
    #[inline(always)]
    pub fn vmaie(&mut self) -> VMAIE_W {
        VMAIE_W { w: self }
    }
    #[doc = "Bit 4 - NMI pin interrupt enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NMIIE_W {
        NMIIE_W { w: self }
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt enable"]
    #[inline(always)]
    pub fn jmbinie(&mut self) -> JMBINIE_W {
        JMBINIE_W { w: self }
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt enable"]
    #[inline(always)]
    pub fn jmboutie(&mut self) -> JMBOUTIE_W {
        JMBOUTIE_W { w: self }
    }
}
