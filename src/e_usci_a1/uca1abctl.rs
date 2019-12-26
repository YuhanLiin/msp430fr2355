#[doc = "Reader of register UCA1ABCTL"]
pub type R = crate::R<u16, super::UCA1ABCTL>;
#[doc = "Writer for register UCA1ABCTL"]
pub type W = crate::W<u16, super::UCA1ABCTL>;
#[doc = "Register UCA1ABCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA1ABCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Automatic baud-rate detect enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCABDEN_A {
    #[doc = "0: Baud-rate detection disabled. Length of break and synch field is not measured."]
    UCABDEN_0 = 0,
    #[doc = "1: Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly."]
    UCABDEN_1 = 1,
}
impl From<UCABDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCABDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCABDEN`"]
pub type UCABDEN_R = crate::R<bool, UCABDEN_A>;
impl UCABDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCABDEN_A {
        match self.bits {
            false => UCABDEN_A::UCABDEN_0,
            true => UCABDEN_A::UCABDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCABDEN_0`"]
    #[inline(always)]
    pub fn is_ucabden_0(&self) -> bool {
        *self == UCABDEN_A::UCABDEN_0
    }
    #[doc = "Checks if the value of the field is `UCABDEN_1`"]
    #[inline(always)]
    pub fn is_ucabden_1(&self) -> bool {
        *self == UCABDEN_A::UCABDEN_1
    }
}
#[doc = "Write proxy for field `UCABDEN`"]
pub struct UCABDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCABDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCABDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baud-rate detection disabled. Length of break and synch field is not measured."]
    #[inline(always)]
    pub fn ucabden_0(self) -> &'a mut W {
        self.variant(UCABDEN_A::UCABDEN_0)
    }
    #[doc = "Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly."]
    #[inline(always)]
    pub fn ucabden_1(self) -> &'a mut W {
        self.variant(UCABDEN_A::UCABDEN_1)
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
#[doc = "Break time out error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBTOE_A {
    #[doc = "0: No error"]
    UCBTOE_0 = 0,
    #[doc = "1: Length of break field exceeded 22 bit times"]
    UCBTOE_1 = 1,
}
impl From<UCBTOE_A> for bool {
    #[inline(always)]
    fn from(variant: UCBTOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCBTOE`"]
pub type UCBTOE_R = crate::R<bool, UCBTOE_A>;
impl UCBTOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBTOE_A {
        match self.bits {
            false => UCBTOE_A::UCBTOE_0,
            true => UCBTOE_A::UCBTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBTOE_0`"]
    #[inline(always)]
    pub fn is_ucbtoe_0(&self) -> bool {
        *self == UCBTOE_A::UCBTOE_0
    }
    #[doc = "Checks if the value of the field is `UCBTOE_1`"]
    #[inline(always)]
    pub fn is_ucbtoe_1(&self) -> bool {
        *self == UCBTOE_A::UCBTOE_1
    }
}
#[doc = "Write proxy for field `UCBTOE`"]
pub struct UCBTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBTOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBTOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucbtoe_0(self) -> &'a mut W {
        self.variant(UCBTOE_A::UCBTOE_0)
    }
    #[doc = "Length of break field exceeded 22 bit times"]
    #[inline(always)]
    pub fn ucbtoe_1(self) -> &'a mut W {
        self.variant(UCBTOE_A::UCBTOE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Synch field time out error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTOE_A {
    #[doc = "0: No error"]
    UCSTOE_0 = 0,
    #[doc = "1: Length of synch field exceeded measurable time"]
    UCSTOE_1 = 1,
}
impl From<UCSTOE_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCSTOE`"]
pub type UCSTOE_R = crate::R<bool, UCSTOE_A>;
impl UCSTOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTOE_A {
        match self.bits {
            false => UCSTOE_A::UCSTOE_0,
            true => UCSTOE_A::UCSTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTOE_0`"]
    #[inline(always)]
    pub fn is_ucstoe_0(&self) -> bool {
        *self == UCSTOE_A::UCSTOE_0
    }
    #[doc = "Checks if the value of the field is `UCSTOE_1`"]
    #[inline(always)]
    pub fn is_ucstoe_1(&self) -> bool {
        *self == UCSTOE_A::UCSTOE_1
    }
}
#[doc = "Write proxy for field `UCSTOE`"]
pub struct UCSTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucstoe_0(self) -> &'a mut W {
        self.variant(UCSTOE_A::UCSTOE_0)
    }
    #[doc = "Length of synch field exceeded measurable time"]
    #[inline(always)]
    pub fn ucstoe_1(self) -> &'a mut W {
        self.variant(UCSTOE_A::UCSTOE_1)
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
#[doc = "Break/synch delimiter length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCDELIM_A {
    #[doc = "0: 1 bit time"]
    UCDELIM_0 = 0,
    #[doc = "1: 2 bit times"]
    UCDELIM_1 = 1,
    #[doc = "2: 3 bit times"]
    UCDELIM_2 = 2,
    #[doc = "3: 4 bit times"]
    UCDELIM_3 = 3,
}
impl From<UCDELIM_A> for u8 {
    #[inline(always)]
    fn from(variant: UCDELIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UCDELIM`"]
pub type UCDELIM_R = crate::R<u8, UCDELIM_A>;
impl UCDELIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCDELIM_A {
        match self.bits {
            0 => UCDELIM_A::UCDELIM_0,
            1 => UCDELIM_A::UCDELIM_1,
            2 => UCDELIM_A::UCDELIM_2,
            3 => UCDELIM_A::UCDELIM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCDELIM_0`"]
    #[inline(always)]
    pub fn is_ucdelim_0(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_0
    }
    #[doc = "Checks if the value of the field is `UCDELIM_1`"]
    #[inline(always)]
    pub fn is_ucdelim_1(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_1
    }
    #[doc = "Checks if the value of the field is `UCDELIM_2`"]
    #[inline(always)]
    pub fn is_ucdelim_2(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_2
    }
    #[doc = "Checks if the value of the field is `UCDELIM_3`"]
    #[inline(always)]
    pub fn is_ucdelim_3(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_3
    }
}
#[doc = "Write proxy for field `UCDELIM`"]
pub struct UCDELIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDELIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCDELIM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 bit time"]
    #[inline(always)]
    pub fn ucdelim_0(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_0)
    }
    #[doc = "2 bit times"]
    #[inline(always)]
    pub fn ucdelim_1(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_1)
    }
    #[doc = "3 bit times"]
    #[inline(always)]
    pub fn ucdelim_2(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_2)
    }
    #[doc = "4 bit times"]
    #[inline(always)]
    pub fn ucdelim_3(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Automatic baud-rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UCABDEN_R {
        UCABDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Break time out error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UCBTOE_R {
        UCBTOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synch field time out error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UCSTOE_R {
        UCSTOE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Break/synch delimiter length"]
    #[inline(always)]
    pub fn ucdelim(&self) -> UCDELIM_R {
        UCDELIM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic baud-rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UCABDEN_W {
        UCABDEN_W { w: self }
    }
    #[doc = "Bit 2 - Break time out error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UCBTOE_W {
        UCBTOE_W { w: self }
    }
    #[doc = "Bit 3 - Synch field time out error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UCSTOE_W {
        UCSTOE_W { w: self }
    }
    #[doc = "Bits 4:5 - Break/synch delimiter length"]
    #[inline(always)]
    pub fn ucdelim(&mut self) -> UCDELIM_W {
        UCDELIM_W { w: self }
    }
}
