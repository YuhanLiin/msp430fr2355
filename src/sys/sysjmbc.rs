#[doc = "Reader of register SYSJMBC"]
pub type R = crate::R<u16, super::SYSJMBC>;
#[doc = "Writer for register SYSJMBC"]
pub type W = crate::W<u16, super::SYSJMBC>;
#[doc = "Register SYSJMBC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSJMBC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Incoming JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBIN0FG_A {
    #[doc = "0: JMBI0 has no new data"]
    JMBIN0FG_0 = 0,
    #[doc = "1: JMBI0 has new data available"]
    JMBIN0FG_1 = 1,
}
impl From<JMBIN0FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBIN0FG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JMBIN0FG`"]
pub type JMBIN0FG_R = crate::R<bool, JMBIN0FG_A>;
impl JMBIN0FG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBIN0FG_A {
        match self.bits {
            false => JMBIN0FG_A::JMBIN0FG_0,
            true => JMBIN0FG_A::JMBIN0FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBIN0FG_0`"]
    #[inline(always)]
    pub fn is_jmbin0fg_0(&self) -> bool {
        *self == JMBIN0FG_A::JMBIN0FG_0
    }
    #[doc = "Checks if the value of the field is `JMBIN0FG_1`"]
    #[inline(always)]
    pub fn is_jmbin0fg_1(&self) -> bool {
        *self == JMBIN0FG_A::JMBIN0FG_1
    }
}
#[doc = "Write proxy for field `JMBIN0FG`"]
pub struct JMBIN0FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBIN0FG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBIN0FG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "JMBI0 has no new data"]
    #[inline(always)]
    pub fn jmbin0fg_0(self) -> &'a mut W {
        self.variant(JMBIN0FG_A::JMBIN0FG_0)
    }
    #[doc = "JMBI0 has new data available"]
    #[inline(always)]
    pub fn jmbin0fg_1(self) -> &'a mut W {
        self.variant(JMBIN0FG_A::JMBIN0FG_1)
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
#[doc = "Incoming JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBIN1FG_A {
    #[doc = "0: JMBI1 has no new data"]
    JMBIN1FG_0 = 0,
    #[doc = "1: JMBI1 has new data available"]
    JMBIN1FG_1 = 1,
}
impl From<JMBIN1FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBIN1FG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JMBIN1FG`"]
pub type JMBIN1FG_R = crate::R<bool, JMBIN1FG_A>;
impl JMBIN1FG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBIN1FG_A {
        match self.bits {
            false => JMBIN1FG_A::JMBIN1FG_0,
            true => JMBIN1FG_A::JMBIN1FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBIN1FG_0`"]
    #[inline(always)]
    pub fn is_jmbin1fg_0(&self) -> bool {
        *self == JMBIN1FG_A::JMBIN1FG_0
    }
    #[doc = "Checks if the value of the field is `JMBIN1FG_1`"]
    #[inline(always)]
    pub fn is_jmbin1fg_1(&self) -> bool {
        *self == JMBIN1FG_A::JMBIN1FG_1
    }
}
#[doc = "Write proxy for field `JMBIN1FG`"]
pub struct JMBIN1FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBIN1FG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBIN1FG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "JMBI1 has no new data"]
    #[inline(always)]
    pub fn jmbin1fg_0(self) -> &'a mut W {
        self.variant(JMBIN1FG_A::JMBIN1FG_0)
    }
    #[doc = "JMBI1 has new data available"]
    #[inline(always)]
    pub fn jmbin1fg_1(self) -> &'a mut W {
        self.variant(JMBIN1FG_A::JMBIN1FG_1)
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
#[doc = "Outgoing JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBOUT0FG_A {
    #[doc = "0: JMBO0 is not ready to receive new data"]
    JMBOUT0FG_0 = 0,
    #[doc = "1: JMBO0 is ready to receive new data"]
    JMBOUT0FG_1 = 1,
}
impl From<JMBOUT0FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUT0FG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JMBOUT0FG`"]
pub type JMBOUT0FG_R = crate::R<bool, JMBOUT0FG_A>;
impl JMBOUT0FG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUT0FG_A {
        match self.bits {
            false => JMBOUT0FG_A::JMBOUT0FG_0,
            true => JMBOUT0FG_A::JMBOUT0FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBOUT0FG_0`"]
    #[inline(always)]
    pub fn is_jmbout0fg_0(&self) -> bool {
        *self == JMBOUT0FG_A::JMBOUT0FG_0
    }
    #[doc = "Checks if the value of the field is `JMBOUT0FG_1`"]
    #[inline(always)]
    pub fn is_jmbout0fg_1(&self) -> bool {
        *self == JMBOUT0FG_A::JMBOUT0FG_1
    }
}
#[doc = "Outgoing JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBOUT1FG_A {
    #[doc = "0: JMBO1 is not ready to receive new data"]
    JMBOUT1FG_0 = 0,
    #[doc = "1: JMBO1 is ready to receive new data"]
    JMBOUT1FG_1 = 1,
}
impl From<JMBOUT1FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUT1FG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JMBOUT1FG`"]
pub type JMBOUT1FG_R = crate::R<bool, JMBOUT1FG_A>;
impl JMBOUT1FG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUT1FG_A {
        match self.bits {
            false => JMBOUT1FG_A::JMBOUT1FG_0,
            true => JMBOUT1FG_A::JMBOUT1FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBOUT1FG_0`"]
    #[inline(always)]
    pub fn is_jmbout1fg_0(&self) -> bool {
        *self == JMBOUT1FG_A::JMBOUT1FG_0
    }
    #[doc = "Checks if the value of the field is `JMBOUT1FG_1`"]
    #[inline(always)]
    pub fn is_jmbout1fg_1(&self) -> bool {
        *self == JMBOUT1FG_A::JMBOUT1FG_1
    }
}
#[doc = "Operation mode of JMB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBMODE_A {
    #[doc = "0: 16-bit transfers using JMBO0 and JMBI0 only"]
    _16BIT = 0,
    #[doc = "1: 32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    _32BIT = 1,
}
impl From<JMBMODE_A> for bool {
    #[inline(always)]
    fn from(variant: JMBMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JMBMODE`"]
pub type JMBMODE_R = crate::R<bool, JMBMODE_A>;
impl JMBMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBMODE_A {
        match self.bits {
            false => JMBMODE_A::_16BIT,
            true => JMBMODE_A::_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == JMBMODE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == JMBMODE_A::_32BIT
    }
}
#[doc = "Write proxy for field `JMBMODE`"]
pub struct JMBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16-bit transfers using JMBO0 and JMBI0 only"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(JMBMODE_A::_16BIT)
    }
    #[doc = "32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(JMBMODE_A::_32BIT)
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
#[doc = "Incoming JTAG Mailbox 0 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBCLR0OFF_A {
    #[doc = "0: JMBIN0FG cleared on read of JMB0IN register"]
    JMBCLR0OFF_0 = 0,
    #[doc = "1: JMBIN0FG cleared by software"]
    JMBCLR0OFF_1 = 1,
}
impl From<JMBCLR0OFF_A> for bool {
    #[inline(always)]
    fn from(variant: JMBCLR0OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JMBCLR0OFF`"]
pub type JMBCLR0OFF_R = crate::R<bool, JMBCLR0OFF_A>;
impl JMBCLR0OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBCLR0OFF_A {
        match self.bits {
            false => JMBCLR0OFF_A::JMBCLR0OFF_0,
            true => JMBCLR0OFF_A::JMBCLR0OFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBCLR0OFF_0`"]
    #[inline(always)]
    pub fn is_jmbclr0off_0(&self) -> bool {
        *self == JMBCLR0OFF_A::JMBCLR0OFF_0
    }
    #[doc = "Checks if the value of the field is `JMBCLR0OFF_1`"]
    #[inline(always)]
    pub fn is_jmbclr0off_1(&self) -> bool {
        *self == JMBCLR0OFF_A::JMBCLR0OFF_1
    }
}
#[doc = "Write proxy for field `JMBCLR0OFF`"]
pub struct JMBCLR0OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBCLR0OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBCLR0OFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "JMBIN0FG cleared on read of JMB0IN register"]
    #[inline(always)]
    pub fn jmbclr0off_0(self) -> &'a mut W {
        self.variant(JMBCLR0OFF_A::JMBCLR0OFF_0)
    }
    #[doc = "JMBIN0FG cleared by software"]
    #[inline(always)]
    pub fn jmbclr0off_1(self) -> &'a mut W {
        self.variant(JMBCLR0OFF_A::JMBCLR0OFF_1)
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
#[doc = "Incoming JTAG Mailbox 1 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBCLR1OFF_A {
    #[doc = "0: JMBIN1FG cleared on read of JMB1IN register"]
    JMBCLR1OFF_0 = 0,
    #[doc = "1: JMBIN1FG cleared by software"]
    JMBCLR1OFF_1 = 1,
}
impl From<JMBCLR1OFF_A> for bool {
    #[inline(always)]
    fn from(variant: JMBCLR1OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JMBCLR1OFF`"]
pub type JMBCLR1OFF_R = crate::R<bool, JMBCLR1OFF_A>;
impl JMBCLR1OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBCLR1OFF_A {
        match self.bits {
            false => JMBCLR1OFF_A::JMBCLR1OFF_0,
            true => JMBCLR1OFF_A::JMBCLR1OFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBCLR1OFF_0`"]
    #[inline(always)]
    pub fn is_jmbclr1off_0(&self) -> bool {
        *self == JMBCLR1OFF_A::JMBCLR1OFF_0
    }
    #[doc = "Checks if the value of the field is `JMBCLR1OFF_1`"]
    #[inline(always)]
    pub fn is_jmbclr1off_1(&self) -> bool {
        *self == JMBCLR1OFF_A::JMBCLR1OFF_1
    }
}
#[doc = "Write proxy for field `JMBCLR1OFF`"]
pub struct JMBCLR1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBCLR1OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBCLR1OFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "JMBIN1FG cleared on read of JMB1IN register"]
    #[inline(always)]
    pub fn jmbclr1off_0(self) -> &'a mut W {
        self.variant(JMBCLR1OFF_A::JMBCLR1OFF_0)
    }
    #[doc = "JMBIN1FG cleared by software"]
    #[inline(always)]
    pub fn jmbclr1off_1(self) -> &'a mut W {
        self.variant(JMBCLR1OFF_A::JMBCLR1OFF_1)
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
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> JMBIN0FG_R {
        JMBIN0FG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> JMBIN1FG_R {
        JMBIN1FG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Outgoing JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> JMBOUT0FG_R {
        JMBOUT0FG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Outgoing JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> JMBOUT1FG_R {
        JMBOUT1FG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JMBMODE_R {
        JMBMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> JMBCLR0OFF_R {
        JMBCLR0OFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> JMBCLR1OFF_R {
        JMBCLR1OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbin0fg(&mut self) -> JMBIN0FG_W {
        JMBIN0FG_W { w: self }
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbin1fg(&mut self) -> JMBIN1FG_W {
        JMBIN1FG_W { w: self }
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    pub fn jmbmode(&mut self) -> JMBMODE_W {
        JMBMODE_W { w: self }
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr0off(&mut self) -> JMBCLR0OFF_W {
        JMBCLR0OFF_W { w: self }
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr1off(&mut self) -> JMBCLR1OFF_W {
        JMBCLR1OFF_W { w: self }
    }
}
