#[doc = "Reader of register CP1CTL0"]
pub type R = crate::R<u16, super::CP1CTL0>;
#[doc = "Writer for register CP1CTL0"]
pub type W = crate::W<u16, super::CP1CTL0>;
#[doc = "Register CP1CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CP1CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel input enable for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPPEN_A {
    #[doc = "0: Selected analog input channel for V+ terminal is disabled."]
    CPPEN_0 = 0,
    #[doc = "1: Selected analog input channel for V+ terminal is enabled."]
    CPPEN_1 = 1,
}
impl From<CPPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPPEN`"]
pub type CPPEN_R = crate::R<bool, CPPEN_A>;
impl CPPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPPEN_A {
        match self.bits {
            false => CPPEN_A::CPPEN_0,
            true => CPPEN_A::CPPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPPEN_0`"]
    #[inline(always)]
    pub fn is_cppen_0(&self) -> bool {
        *self == CPPEN_A::CPPEN_0
    }
    #[doc = "Checks if the value of the field is `CPPEN_1`"]
    #[inline(always)]
    pub fn is_cppen_1(&self) -> bool {
        *self == CPPEN_A::CPPEN_1
    }
}
#[doc = "Write proxy for field `CPPEN`"]
pub struct CPPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected analog input channel for V+ terminal is disabled."]
    #[inline(always)]
    pub fn cppen_0(self) -> &'a mut W {
        self.variant(CPPEN_A::CPPEN_0)
    }
    #[doc = "Selected analog input channel for V+ terminal is enabled."]
    #[inline(always)]
    pub fn cppen_1(self) -> &'a mut W {
        self.variant(CPPEN_A::CPPEN_1)
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
#[doc = "Channel input selected for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPNSEL_A {
    #[doc = "0: select external input source"]
    CPNSEL_0 = 0,
    #[doc = "1: select external input source"]
    CPNSEL_1 = 1,
    #[doc = "2: select external input source"]
    CPNSEL_2 = 2,
    #[doc = "3: select external input source"]
    CPNSEL_3 = 3,
    #[doc = "4: device specific, please refer to device data sheet for details"]
    CPNSEL_4 = 4,
    #[doc = "5: device specific, please refer to device data sheet for details"]
    CPNSEL_5 = 5,
    #[doc = "6: 6-bit DAC"]
    CPNSEL_6 = 6,
    #[doc = "7: Reserved"]
    CPNSEL_7 = 7,
}
impl From<CPNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPNSEL`"]
pub type CPNSEL_R = crate::R<u8, CPNSEL_A>;
impl CPNSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPNSEL_A {
        match self.bits {
            0 => CPNSEL_A::CPNSEL_0,
            1 => CPNSEL_A::CPNSEL_1,
            2 => CPNSEL_A::CPNSEL_2,
            3 => CPNSEL_A::CPNSEL_3,
            4 => CPNSEL_A::CPNSEL_4,
            5 => CPNSEL_A::CPNSEL_5,
            6 => CPNSEL_A::CPNSEL_6,
            7 => CPNSEL_A::CPNSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPNSEL_0`"]
    #[inline(always)]
    pub fn is_cpnsel_0(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_0
    }
    #[doc = "Checks if the value of the field is `CPNSEL_1`"]
    #[inline(always)]
    pub fn is_cpnsel_1(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_1
    }
    #[doc = "Checks if the value of the field is `CPNSEL_2`"]
    #[inline(always)]
    pub fn is_cpnsel_2(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_2
    }
    #[doc = "Checks if the value of the field is `CPNSEL_3`"]
    #[inline(always)]
    pub fn is_cpnsel_3(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_3
    }
    #[doc = "Checks if the value of the field is `CPNSEL_4`"]
    #[inline(always)]
    pub fn is_cpnsel_4(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_4
    }
    #[doc = "Checks if the value of the field is `CPNSEL_5`"]
    #[inline(always)]
    pub fn is_cpnsel_5(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_5
    }
    #[doc = "Checks if the value of the field is `CPNSEL_6`"]
    #[inline(always)]
    pub fn is_cpnsel_6(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_6
    }
    #[doc = "Checks if the value of the field is `CPNSEL_7`"]
    #[inline(always)]
    pub fn is_cpnsel_7(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_7
    }
}
#[doc = "Write proxy for field `CPNSEL`"]
pub struct CPNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPNSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_0(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_0)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_1(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_1)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_2(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_2)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_3(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_3)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cpnsel_4(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_4)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cpnsel_5(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_5)
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn cpnsel_6(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn cpnsel_7(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Channel input enable for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPNEN_A {
    #[doc = "0: Selected analog input channel for V- terminal is disabled."]
    CPNEN_0 = 0,
    #[doc = "1: Selected analog input channel for V- terminal is enabled."]
    CPNEN_1 = 1,
}
impl From<CPNEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPNEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPNEN`"]
pub type CPNEN_R = crate::R<bool, CPNEN_A>;
impl CPNEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPNEN_A {
        match self.bits {
            false => CPNEN_A::CPNEN_0,
            true => CPNEN_A::CPNEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPNEN_0`"]
    #[inline(always)]
    pub fn is_cpnen_0(&self) -> bool {
        *self == CPNEN_A::CPNEN_0
    }
    #[doc = "Checks if the value of the field is `CPNEN_1`"]
    #[inline(always)]
    pub fn is_cpnen_1(&self) -> bool {
        *self == CPNEN_A::CPNEN_1
    }
}
#[doc = "Write proxy for field `CPNEN`"]
pub struct CPNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPNEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected analog input channel for V- terminal is disabled."]
    #[inline(always)]
    pub fn cpnen_0(self) -> &'a mut W {
        self.variant(CPNEN_A::CPNEN_0)
    }
    #[doc = "Selected analog input channel for V- terminal is enabled."]
    #[inline(always)]
    pub fn cpnen_1(self) -> &'a mut W {
        self.variant(CPNEN_A::CPNEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Channel input selected for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPPSEL_A {
    #[doc = "0: select external input source"]
    CPPSEL_0 = 0,
    #[doc = "1: select external input source"]
    CPPSEL_1 = 1,
    #[doc = "2: select external input source"]
    CPPSEL_2 = 2,
    #[doc = "3: select external input source"]
    CPPSEL_3 = 3,
    #[doc = "4: device specific, please refer to device data sheet for details"]
    CPPSEL_4 = 4,
    #[doc = "5: device specific, please refer to device data sheet for details"]
    CPPSEL_5 = 5,
    #[doc = "6: 6-bit DAC"]
    CPPSEL_6 = 6,
    #[doc = "7: Reserved"]
    CPPSEL_7 = 7,
}
impl From<CPPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPPSEL`"]
pub type CPPSEL_R = crate::R<u8, CPPSEL_A>;
impl CPPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPPSEL_A {
        match self.bits {
            0 => CPPSEL_A::CPPSEL_0,
            1 => CPPSEL_A::CPPSEL_1,
            2 => CPPSEL_A::CPPSEL_2,
            3 => CPPSEL_A::CPPSEL_3,
            4 => CPPSEL_A::CPPSEL_4,
            5 => CPPSEL_A::CPPSEL_5,
            6 => CPPSEL_A::CPPSEL_6,
            7 => CPPSEL_A::CPPSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPPSEL_0`"]
    #[inline(always)]
    pub fn is_cppsel_0(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_0
    }
    #[doc = "Checks if the value of the field is `CPPSEL_1`"]
    #[inline(always)]
    pub fn is_cppsel_1(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_1
    }
    #[doc = "Checks if the value of the field is `CPPSEL_2`"]
    #[inline(always)]
    pub fn is_cppsel_2(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_2
    }
    #[doc = "Checks if the value of the field is `CPPSEL_3`"]
    #[inline(always)]
    pub fn is_cppsel_3(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_3
    }
    #[doc = "Checks if the value of the field is `CPPSEL_4`"]
    #[inline(always)]
    pub fn is_cppsel_4(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_4
    }
    #[doc = "Checks if the value of the field is `CPPSEL_5`"]
    #[inline(always)]
    pub fn is_cppsel_5(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_5
    }
    #[doc = "Checks if the value of the field is `CPPSEL_6`"]
    #[inline(always)]
    pub fn is_cppsel_6(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_6
    }
    #[doc = "Checks if the value of the field is `CPPSEL_7`"]
    #[inline(always)]
    pub fn is_cppsel_7(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_7
    }
}
#[doc = "Write proxy for field `CPPSEL`"]
pub struct CPPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPPSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_0(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_0)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_1(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_1)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_2(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_2)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_3(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_3)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cppsel_4(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_4)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cppsel_5(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_5)
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn cppsel_6(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn cppsel_7(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn cppen(&self) -> CPPEN_R {
        CPPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn cpnsel(&self) -> CPNSEL_R {
        CPNSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn cpnen(&self) -> CPNEN_R {
        CPNEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn cppsel(&self) -> CPPSEL_R {
        CPPSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn cppen(&mut self) -> CPPEN_W {
        CPPEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn cpnsel(&mut self) -> CPNSEL_W {
        CPNSEL_W { w: self }
    }
    #[doc = "Bit 12 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn cpnen(&mut self) -> CPNEN_W {
        CPNEN_W { w: self }
    }
    #[doc = "Bits 0:2 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn cppsel(&mut self) -> CPPSEL_W {
        CPPSEL_W { w: self }
    }
}
