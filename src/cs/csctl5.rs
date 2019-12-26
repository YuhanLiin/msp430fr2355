#[doc = "Reader of register CSCTL5"]
pub type R = crate::R<u16, super::CSCTL5>;
#[doc = "Writer for register CSCTL5"]
pub type W = crate::W<u16, super::CSCTL5>;
#[doc = "Register CSCTL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL5 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
    #[doc = "4: /16"]
    _16 = 4,
    #[doc = "5: /32"]
    _32 = 5,
    #[doc = "6: /64"]
    _64 = 6,
    #[doc = "7: /128"]
    _128 = 7,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVM`"]
pub type DIVM_R = crate::R<u8, DIVM_A>;
impl DIVM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVM_A {
        match self.bits {
            0 => DIVM_A::_1,
            1 => DIVM_A::_2,
            2 => DIVM_A::_4,
            3 => DIVM_A::_8,
            4 => DIVM_A::_16,
            5 => DIVM_A::_32,
            6 => DIVM_A::_64,
            7 => DIVM_A::_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIVM_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == DIVM_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DIVM_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DIVM_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DIVM_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == DIVM_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == DIVM_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == DIVM_A::_128
    }
}
#[doc = "Write proxy for field `DIVM`"]
pub struct DIVM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIVM_A::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(DIVM_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(DIVM_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DIVM_A::_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DIVM_A::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(DIVM_A::_32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(DIVM_A::_64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(DIVM_A::_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVS`"]
pub type DIVS_R = crate::R<u8, DIVS_A>;
impl DIVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVS_A {
        match self.bits {
            0 => DIVS_A::_1,
            1 => DIVS_A::_2,
            2 => DIVS_A::_4,
            3 => DIVS_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIVS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == DIVS_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DIVS_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DIVS_A::_8
    }
}
#[doc = "Write proxy for field `DIVS`"]
pub struct DIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIVS_A::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(DIVS_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(DIVS_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DIVS_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "SMCLK off. This bit turns off SMCLK clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLKOFF_A {
    #[doc = "0: SMCLK on"]
    SMCLKOFF_0 = 0,
    #[doc = "1: SMCLK off"]
    SMCLKOFF_1 = 1,
}
impl From<SMCLKOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLKOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMCLKOFF`"]
pub type SMCLKOFF_R = crate::R<bool, SMCLKOFF_A>;
impl SMCLKOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLKOFF_A {
        match self.bits {
            false => SMCLKOFF_A::SMCLKOFF_0,
            true => SMCLKOFF_A::SMCLKOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLKOFF_0`"]
    #[inline(always)]
    pub fn is_smclkoff_0(&self) -> bool {
        *self == SMCLKOFF_A::SMCLKOFF_0
    }
    #[doc = "Checks if the value of the field is `SMCLKOFF_1`"]
    #[inline(always)]
    pub fn is_smclkoff_1(&self) -> bool {
        *self == SMCLKOFF_A::SMCLKOFF_1
    }
}
#[doc = "Write proxy for field `SMCLKOFF`"]
pub struct SMCLKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMCLKOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMCLK on"]
    #[inline(always)]
    pub fn smclkoff_0(self) -> &'a mut W {
        self.variant(SMCLKOFF_A::SMCLKOFF_0)
    }
    #[doc = "SMCLK off"]
    #[inline(always)]
    pub fn smclkoff_1(self) -> &'a mut W {
        self.variant(SMCLKOFF_A::SMCLKOFF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "VLO automatic off enable. This bit turns off VLO, if VLO is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLOAUTOOFF_A {
    #[doc = "0: VLO always on"]
    VLOAUTOOFF_0 = 0,
    #[doc = "1: VLO automatically turned off if not used(default)"]
    VLOAUTOOFF_1 = 1,
}
impl From<VLOAUTOOFF_A> for bool {
    #[inline(always)]
    fn from(variant: VLOAUTOOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VLOAUTOOFF`"]
pub type VLOAUTOOFF_R = crate::R<bool, VLOAUTOOFF_A>;
impl VLOAUTOOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLOAUTOOFF_A {
        match self.bits {
            false => VLOAUTOOFF_A::VLOAUTOOFF_0,
            true => VLOAUTOOFF_A::VLOAUTOOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLOAUTOOFF_0`"]
    #[inline(always)]
    pub fn is_vloautooff_0(&self) -> bool {
        *self == VLOAUTOOFF_A::VLOAUTOOFF_0
    }
    #[doc = "Checks if the value of the field is `VLOAUTOOFF_1`"]
    #[inline(always)]
    pub fn is_vloautooff_1(&self) -> bool {
        *self == VLOAUTOOFF_A::VLOAUTOOFF_1
    }
}
#[doc = "Write proxy for field `VLOAUTOOFF`"]
pub struct VLOAUTOOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VLOAUTOOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLOAUTOOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VLO always on"]
    #[inline(always)]
    pub fn vloautooff_0(self) -> &'a mut W {
        self.variant(VLOAUTOOFF_A::VLOAUTOOFF_0)
    }
    #[doc = "VLO automatically turned off if not used(default)"]
    #[inline(always)]
    pub fn vloautooff_1(self) -> &'a mut W {
        self.variant(VLOAUTOOFF_A::VLOAUTOOFF_1)
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
impl R {
    #[doc = "Bits 0:2 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source."]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - SMCLK off. This bit turns off SMCLK clock"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - VLO automatic off enable. This bit turns off VLO, if VLO is not used."]
    #[inline(always)]
    pub fn vloautooff(&self) -> VLOAUTOOFF_R {
        VLOAUTOOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W {
        DIVM_W { w: self }
    }
    #[doc = "Bits 4:5 - SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source."]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W {
        DIVS_W { w: self }
    }
    #[doc = "Bit 8 - SMCLK off. This bit turns off SMCLK clock"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W {
        SMCLKOFF_W { w: self }
    }
    #[doc = "Bit 12 - VLO automatic off enable. This bit turns off VLO, if VLO is not used."]
    #[inline(always)]
    pub fn vloautooff(&mut self) -> VLOAUTOOFF_W {
        VLOAUTOOFF_W { w: self }
    }
}
