#[doc = "Reader of register MPY32CTL0"]
pub type R = crate::R<u16, super::MPY32CTL0>;
#[doc = "Writer for register MPY32CTL0"]
pub type W = crate::W<u16, super::MPY32CTL0>;
#[doc = "Register MPY32CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPY32CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Delayed write mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYDLY32_A {
    #[doc = "0: Writes are delayed until 64-bit result (RES0 to RES3) is available."]
    MPYDLY32_0 = 0,
    #[doc = "1: Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN"]
    MPYDLY32_1 = 1,
}
impl From<MPYDLY32_A> for bool {
    #[inline(always)]
    fn from(variant: MPYDLY32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPYDLY32`"]
pub type MPYDLY32_R = crate::R<bool, MPYDLY32_A>;
impl MPYDLY32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYDLY32_A {
        match self.bits {
            false => MPYDLY32_A::MPYDLY32_0,
            true => MPYDLY32_A::MPYDLY32_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPYDLY32_0`"]
    #[inline(always)]
    pub fn is_mpydly32_0(&self) -> bool {
        *self == MPYDLY32_A::MPYDLY32_0
    }
    #[doc = "Checks if the value of the field is `MPYDLY32_1`"]
    #[inline(always)]
    pub fn is_mpydly32_1(&self) -> bool {
        *self == MPYDLY32_A::MPYDLY32_1
    }
}
#[doc = "Write proxy for field `MPYDLY32`"]
pub struct MPYDLY32_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYDLY32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYDLY32_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes are delayed until 64-bit result (RES0 to RES3) is available."]
    #[inline(always)]
    pub fn mpydly32_0(self) -> &'a mut W {
        self.variant(MPYDLY32_A::MPYDLY32_0)
    }
    #[doc = "Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN"]
    #[inline(always)]
    pub fn mpydly32_1(self) -> &'a mut W {
        self.variant(MPYDLY32_A::MPYDLY32_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Delayed write enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYDLYWRTEN_A {
    #[doc = "0: Writes are not delayed."]
    MPYDLYWRTEN_0 = 0,
    #[doc = "1: Writes are delayed."]
    MPYDLYWRTEN_1 = 1,
}
impl From<MPYDLYWRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MPYDLYWRTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPYDLYWRTEN`"]
pub type MPYDLYWRTEN_R = crate::R<bool, MPYDLYWRTEN_A>;
impl MPYDLYWRTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYDLYWRTEN_A {
        match self.bits {
            false => MPYDLYWRTEN_A::MPYDLYWRTEN_0,
            true => MPYDLYWRTEN_A::MPYDLYWRTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPYDLYWRTEN_0`"]
    #[inline(always)]
    pub fn is_mpydlywrten_0(&self) -> bool {
        *self == MPYDLYWRTEN_A::MPYDLYWRTEN_0
    }
    #[doc = "Checks if the value of the field is `MPYDLYWRTEN_1`"]
    #[inline(always)]
    pub fn is_mpydlywrten_1(&self) -> bool {
        *self == MPYDLYWRTEN_A::MPYDLYWRTEN_1
    }
}
#[doc = "Write proxy for field `MPYDLYWRTEN`"]
pub struct MPYDLYWRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYDLYWRTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYDLYWRTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes are not delayed."]
    #[inline(always)]
    pub fn mpydlywrten_0(self) -> &'a mut W {
        self.variant(MPYDLYWRTEN_A::MPYDLYWRTEN_0)
    }
    #[doc = "Writes are delayed."]
    #[inline(always)]
    pub fn mpydlywrten_1(self) -> &'a mut W {
        self.variant(MPYDLYWRTEN_A::MPYDLYWRTEN_1)
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
#[doc = "Multiplier bit width of operand 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYOP2_32_A {
    #[doc = "0: 16 bits."]
    _16 = 0,
    #[doc = "1: 32 bits."]
    _32 = 1,
}
impl From<MPYOP2_32_A> for bool {
    #[inline(always)]
    fn from(variant: MPYOP2_32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPYOP2_32`"]
pub type MPYOP2_32_R = crate::R<bool, MPYOP2_32_A>;
impl MPYOP2_32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYOP2_32_A {
        match self.bits {
            false => MPYOP2_32_A::_16,
            true => MPYOP2_32_A::_32,
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == MPYOP2_32_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == MPYOP2_32_A::_32
    }
}
#[doc = "Write proxy for field `MPYOP2_32`"]
pub struct MPYOP2_32_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYOP2_32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYOP2_32_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(MPYOP2_32_A::_16)
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(MPYOP2_32_A::_32)
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
#[doc = "Multiplier bit width of operand 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYOP1_32_A {
    #[doc = "0: 16 bits."]
    _16 = 0,
    #[doc = "1: 32 bits."]
    _32 = 1,
}
impl From<MPYOP1_32_A> for bool {
    #[inline(always)]
    fn from(variant: MPYOP1_32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPYOP1_32`"]
pub type MPYOP1_32_R = crate::R<bool, MPYOP1_32_A>;
impl MPYOP1_32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYOP1_32_A {
        match self.bits {
            false => MPYOP1_32_A::_16,
            true => MPYOP1_32_A::_32,
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == MPYOP1_32_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == MPYOP1_32_A::_32
    }
}
#[doc = "Write proxy for field `MPYOP1_32`"]
pub struct MPYOP1_32_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYOP1_32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYOP1_32_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(MPYOP1_32_A::_16)
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(MPYOP1_32_A::_32)
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
#[doc = "Multiplier mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPYM_A {
    #[doc = "0: MPY  Multiply"]
    MPY = 0,
    #[doc = "1: MPYS  Signed multiply"]
    MPYS = 1,
    #[doc = "2: MAC  Multiply accumulate"]
    MAC = 2,
    #[doc = "3: MACS  Signed multiply accumulate"]
    MACS = 3,
}
impl From<MPYM_A> for u8 {
    #[inline(always)]
    fn from(variant: MPYM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPYM`"]
pub type MPYM_R = crate::R<u8, MPYM_A>;
impl MPYM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYM_A {
        match self.bits {
            0 => MPYM_A::MPY,
            1 => MPYM_A::MPYS,
            2 => MPYM_A::MAC,
            3 => MPYM_A::MACS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MPY`"]
    #[inline(always)]
    pub fn is_mpy(&self) -> bool {
        *self == MPYM_A::MPY
    }
    #[doc = "Checks if the value of the field is `MPYS`"]
    #[inline(always)]
    pub fn is_mpys(&self) -> bool {
        *self == MPYM_A::MPYS
    }
    #[doc = "Checks if the value of the field is `MAC`"]
    #[inline(always)]
    pub fn is_mac(&self) -> bool {
        *self == MPYM_A::MAC
    }
    #[doc = "Checks if the value of the field is `MACS`"]
    #[inline(always)]
    pub fn is_macs(&self) -> bool {
        *self == MPYM_A::MACS
    }
}
#[doc = "Write proxy for field `MPYM`"]
pub struct MPYM_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MPY Multiply"]
    #[inline(always)]
    pub fn mpy(self) -> &'a mut W {
        self.variant(MPYM_A::MPY)
    }
    #[doc = "MPYS Signed multiply"]
    #[inline(always)]
    pub fn mpys(self) -> &'a mut W {
        self.variant(MPYM_A::MPYS)
    }
    #[doc = "MAC Multiply accumulate"]
    #[inline(always)]
    pub fn mac(self) -> &'a mut W {
        self.variant(MPYM_A::MAC)
    }
    #[doc = "MACS Signed multiply accumulate"]
    #[inline(always)]
    pub fn macs(self) -> &'a mut W {
        self.variant(MPYM_A::MACS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Saturation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYSAT_A {
    #[doc = "0: Saturation mode disabled."]
    DISABLE = 0,
    #[doc = "1: Saturation mode enabled."]
    ENABLE = 1,
}
impl From<MPYSAT_A> for bool {
    #[inline(always)]
    fn from(variant: MPYSAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPYSAT`"]
pub type MPYSAT_R = crate::R<bool, MPYSAT_A>;
impl MPYSAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYSAT_A {
        match self.bits {
            false => MPYSAT_A::DISABLE,
            true => MPYSAT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPYSAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPYSAT_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPYSAT`"]
pub struct MPYSAT_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYSAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYSAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Saturation mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPYSAT_A::DISABLE)
    }
    #[doc = "Saturation mode enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPYSAT_A::ENABLE)
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
#[doc = "Fractional mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYFRAC_A {
    #[doc = "0: Fractional mode disabled."]
    DISABLE = 0,
    #[doc = "1: Fractional mode enabled."]
    ENABLE = 1,
}
impl From<MPYFRAC_A> for bool {
    #[inline(always)]
    fn from(variant: MPYFRAC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPYFRAC`"]
pub type MPYFRAC_R = crate::R<bool, MPYFRAC_A>;
impl MPYFRAC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYFRAC_A {
        match self.bits {
            false => MPYFRAC_A::DISABLE,
            true => MPYFRAC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPYFRAC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPYFRAC_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPYFRAC`"]
pub struct MPYFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYFRAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYFRAC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fractional mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPYFRAC_A::DISABLE)
    }
    #[doc = "Fractional mode enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPYFRAC_A::ENABLE)
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
#[doc = "Carry of the multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYC_A {
    #[doc = "0: No carry for result."]
    MPYC_0 = 0,
    #[doc = "1: Result has a carry."]
    MPYC_1 = 1,
}
impl From<MPYC_A> for bool {
    #[inline(always)]
    fn from(variant: MPYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPYC`"]
pub type MPYC_R = crate::R<bool, MPYC_A>;
impl MPYC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYC_A {
        match self.bits {
            false => MPYC_A::MPYC_0,
            true => MPYC_A::MPYC_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPYC_0`"]
    #[inline(always)]
    pub fn is_mpyc_0(&self) -> bool {
        *self == MPYC_A::MPYC_0
    }
    #[doc = "Checks if the value of the field is `MPYC_1`"]
    #[inline(always)]
    pub fn is_mpyc_1(&self) -> bool {
        *self == MPYC_A::MPYC_1
    }
}
#[doc = "Write proxy for field `MPYC`"]
pub struct MPYC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No carry for result."]
    #[inline(always)]
    pub fn mpyc_0(self) -> &'a mut W {
        self.variant(MPYC_A::MPYC_0)
    }
    #[doc = "Result has a carry."]
    #[inline(always)]
    pub fn mpyc_1(self) -> &'a mut W {
        self.variant(MPYC_A::MPYC_1)
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
impl R {
    #[doc = "Bit 9 - Delayed write mode."]
    #[inline(always)]
    pub fn mpydly32(&self) -> MPYDLY32_R {
        MPYDLY32_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Delayed write enable."]
    #[inline(always)]
    pub fn mpydlywrten(&self) -> MPYDLYWRTEN_R {
        MPYDLYWRTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Multiplier bit width of operand 2"]
    #[inline(always)]
    pub fn mpyop2_32(&self) -> MPYOP2_32_R {
        MPYOP2_32_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multiplier bit width of operand 1"]
    #[inline(always)]
    pub fn mpyop1_32(&self) -> MPYOP1_32_R {
        MPYOP1_32_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Multiplier mode"]
    #[inline(always)]
    pub fn mpym(&self) -> MPYM_R {
        MPYM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&self) -> MPYSAT_R {
        MPYSAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fractional mode."]
    #[inline(always)]
    pub fn mpyfrac(&self) -> MPYFRAC_R {
        MPYFRAC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&self) -> MPYC_R {
        MPYC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Delayed write mode."]
    #[inline(always)]
    pub fn mpydly32(&mut self) -> MPYDLY32_W {
        MPYDLY32_W { w: self }
    }
    #[doc = "Bit 8 - Delayed write enable."]
    #[inline(always)]
    pub fn mpydlywrten(&mut self) -> MPYDLYWRTEN_W {
        MPYDLYWRTEN_W { w: self }
    }
    #[doc = "Bit 7 - Multiplier bit width of operand 2"]
    #[inline(always)]
    pub fn mpyop2_32(&mut self) -> MPYOP2_32_W {
        MPYOP2_32_W { w: self }
    }
    #[doc = "Bit 6 - Multiplier bit width of operand 1"]
    #[inline(always)]
    pub fn mpyop1_32(&mut self) -> MPYOP1_32_W {
        MPYOP1_32_W { w: self }
    }
    #[doc = "Bits 4:5 - Multiplier mode"]
    #[inline(always)]
    pub fn mpym(&mut self) -> MPYM_W {
        MPYM_W { w: self }
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&mut self) -> MPYSAT_W {
        MPYSAT_W { w: self }
    }
    #[doc = "Bit 2 - Fractional mode."]
    #[inline(always)]
    pub fn mpyfrac(&mut self) -> MPYFRAC_W {
        MPYFRAC_W { w: self }
    }
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&mut self) -> MPYC_W {
        MPYC_W { w: self }
    }
}
