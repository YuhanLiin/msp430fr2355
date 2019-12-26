#[doc = "Reader of register TB2CTL"]
pub type R = crate::R<u16, super::TB2CTL>;
#[doc = "Writer for register TB2CTL"]
pub type W = crate::W<u16, super::TB2CTL>;
#[doc = "Register TB2CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TB2CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TimerB interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIFG_A {
    #[doc = "0: No interrupt pending"]
    TBIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    TBIFG_1 = 1,
}
impl From<TBIFG_A> for bool {
    #[inline(always)]
    fn from(variant: TBIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBIFG`"]
pub type TBIFG_R = crate::R<bool, TBIFG_A>;
impl TBIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBIFG_A {
        match self.bits {
            false => TBIFG_A::TBIFG_0,
            true => TBIFG_A::TBIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TBIFG_0`"]
    #[inline(always)]
    pub fn is_tbifg_0(&self) -> bool {
        *self == TBIFG_A::TBIFG_0
    }
    #[doc = "Checks if the value of the field is `TBIFG_1`"]
    #[inline(always)]
    pub fn is_tbifg_1(&self) -> bool {
        *self == TBIFG_A::TBIFG_1
    }
}
#[doc = "Write proxy for field `TBIFG`"]
pub struct TBIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn tbifg_0(self) -> &'a mut W {
        self.variant(TBIFG_A::TBIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn tbifg_1(self) -> &'a mut W {
        self.variant(TBIFG_A::TBIFG_1)
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
#[doc = "TimerB interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBIE_A {
    #[doc = "0: Interrupt disabled"]
    TBIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    TBIE_1 = 1,
}
impl From<TBIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBIE`"]
pub type TBIE_R = crate::R<bool, TBIE_A>;
impl TBIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBIE_A {
        match self.bits {
            false => TBIE_A::TBIE_0,
            true => TBIE_A::TBIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TBIE_0`"]
    #[inline(always)]
    pub fn is_tbie_0(&self) -> bool {
        *self == TBIE_A::TBIE_0
    }
    #[doc = "Checks if the value of the field is `TBIE_1`"]
    #[inline(always)]
    pub fn is_tbie_1(&self) -> bool {
        *self == TBIE_A::TBIE_1
    }
}
#[doc = "Write proxy for field `TBIE`"]
pub struct TBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn tbie_0(self) -> &'a mut W {
        self.variant(TBIE_A::TBIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn tbie_1(self) -> &'a mut W {
        self.variant(TBIE_A::TBIE_1)
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
#[doc = "Reader of field `TBCLR`"]
pub type TBCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCLR`"]
pub struct TBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCLR_W<'a> {
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
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Stop mode: Timer is halted"]
    STOP = 0,
    #[doc = "1: Up mode: Timer counts up to TBxCL0"]
    UP = 1,
    #[doc = "2: Continuous mode: Timer counts up to the value set by CNTL"]
    CONTINUOUS = 2,
    #[doc = "3: Up/down mode: Timer counts up to TBxCL0 then down to 0000h"]
    UPDOWN = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u8, MC_A>;
impl MC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::STOP,
            1 => MC_A::UP,
            2 => MC_A::CONTINUOUS,
            3 => MC_A::UPDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MC_A::STOP
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == MC_A::UP
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == MC_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == MC_A::UPDOWN
    }
}
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MC_A::STOP)
    }
    #[doc = "Up mode: Timer counts up to TBxCL0"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(MC_A::UP)
    }
    #[doc = "Continuous mode: Timer counts up to the value set by CNTL"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(MC_A::CONTINUOUS)
    }
    #[doc = "Up/down mode: Timer counts up to TBxCL0 then down to 0000h"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut W {
        self.variant(MC_A::UPDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::_1,
            1 => ID_A::_2,
            2 => ID_A::_4,
            3 => ID_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ID_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == ID_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == ID_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == ID_A::_8
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ID_A::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ID_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ID_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(ID_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "TimerB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBSSEL_A {
    #[doc = "0: TBxCLK"]
    TBCLK = 0,
    #[doc = "1: ACLK"]
    ACLK = 1,
    #[doc = "2: SMCLK"]
    SMCLK = 2,
    #[doc = "3: INCLK"]
    INCLK = 3,
}
impl From<TBSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TBSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBSSEL`"]
pub type TBSSEL_R = crate::R<u8, TBSSEL_A>;
impl TBSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBSSEL_A {
        match self.bits {
            0 => TBSSEL_A::TBCLK,
            1 => TBSSEL_A::ACLK,
            2 => TBSSEL_A::SMCLK,
            3 => TBSSEL_A::INCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TBCLK`"]
    #[inline(always)]
    pub fn is_tbclk(&self) -> bool {
        *self == TBSSEL_A::TBCLK
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == TBSSEL_A::ACLK
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == TBSSEL_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `INCLK`"]
    #[inline(always)]
    pub fn is_inclk(&self) -> bool {
        *self == TBSSEL_A::INCLK
    }
}
#[doc = "Write proxy for field `TBSSEL`"]
pub struct TBSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TBxCLK"]
    #[inline(always)]
    pub fn tbclk(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBCLK)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(TBSSEL_A::ACLK)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(TBSSEL_A::SMCLK)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn inclk(self) -> &'a mut W {
        self.variant(TBSSEL_A::INCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Counter length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTL_A {
    #[doc = "0: 16-bit, TBxR(max) = 0FFFFh"]
    _16 = 0,
    #[doc = "1: 12-bit, TBxR(max) = 0FFFh"]
    _12 = 1,
    #[doc = "2: 10-bit, TBxR(max) = 03FFh"]
    _10 = 2,
    #[doc = "3: 8-bit, TBxR(max) = 0FFh"]
    _8 = 3,
}
impl From<CNTL_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CNTL`"]
pub type CNTL_R = crate::R<u8, CNTL_A>;
impl CNTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTL_A {
        match self.bits {
            0 => CNTL_A::_16,
            1 => CNTL_A::_12,
            2 => CNTL_A::_10,
            3 => CNTL_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CNTL_A::_16
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == CNTL_A::_12
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CNTL_A::_10
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CNTL_A::_8
    }
}
#[doc = "Write proxy for field `CNTL`"]
pub struct CNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "16-bit, TBxR(max) = 0FFFFh"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CNTL_A::_16)
    }
    #[doc = "12-bit, TBxR(max) = 0FFFh"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(CNTL_A::_12)
    }
    #[doc = "10-bit, TBxR(max) = 03FFh"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CNTL_A::_10)
    }
    #[doc = "8-bit, TBxR(max) = 0FFh"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CNTL_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u16) & 0x03) << 11);
        self.w
    }
}
#[doc = "TBxCLn group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBCLGRP_A {
    #[doc = "0: Each TBxCLn latch loads independently"]
    TBCLGRP_0 = 0,
    #[doc = "1: TBxCL1+TBxCL2 (TBxCCR1 CLLD bits control the update); TBxCL3+TBxCL4 (TBxCCR3 CLLD bits control the update); TBxCL5+TBxCL6 (TBxCCR5 CLLD bits control the update); TBxCL0 independent"]
    TBCLGRP_1 = 1,
    #[doc = "2: TBxCL1+TBxCL2+TBxCL3 (TBxCCR1 CLLD bits control the update); TBxCL4+TBxCL5+TBxCL6 (TBxCCR4 CLLD bits control the update); TBxCL0 independent"]
    TBCLGRP_2 = 2,
    #[doc = "3: TBxCL0+TBxCL1+TBxCL2+TBxCL3+TBxCL4+TBxCL5+TBxCL6 (TBxCCR1 CLLD bits control the update)"]
    TBCLGRP_3 = 3,
}
impl From<TBCLGRP_A> for u8 {
    #[inline(always)]
    fn from(variant: TBCLGRP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBCLGRP`"]
pub type TBCLGRP_R = crate::R<u8, TBCLGRP_A>;
impl TBCLGRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCLGRP_A {
        match self.bits {
            0 => TBCLGRP_A::TBCLGRP_0,
            1 => TBCLGRP_A::TBCLGRP_1,
            2 => TBCLGRP_A::TBCLGRP_2,
            3 => TBCLGRP_A::TBCLGRP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_0`"]
    #[inline(always)]
    pub fn is_tbclgrp_0(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_0
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_1`"]
    #[inline(always)]
    pub fn is_tbclgrp_1(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_1
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_2`"]
    #[inline(always)]
    pub fn is_tbclgrp_2(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_2
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_3`"]
    #[inline(always)]
    pub fn is_tbclgrp_3(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_3
    }
}
#[doc = "Write proxy for field `TBCLGRP`"]
pub struct TBCLGRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCLGRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCLGRP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Each TBxCLn latch loads independently"]
    #[inline(always)]
    pub fn tbclgrp_0(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_0)
    }
    #[doc = "TBxCL1+TBxCL2 (TBxCCR1 CLLD bits control the update); TBxCL3+TBxCL4 (TBxCCR3 CLLD bits control the update); TBxCL5+TBxCL6 (TBxCCR5 CLLD bits control the update); TBxCL0 independent"]
    #[inline(always)]
    pub fn tbclgrp_1(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_1)
    }
    #[doc = "TBxCL1+TBxCL2+TBxCL3 (TBxCCR1 CLLD bits control the update); TBxCL4+TBxCL5+TBxCL6 (TBxCCR4 CLLD bits control the update); TBxCL0 independent"]
    #[inline(always)]
    pub fn tbclgrp_2(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_2)
    }
    #[doc = "TBxCL0+TBxCL1+TBxCL2+TBxCL3+TBxCL4+TBxCL5+TBxCL6 (TBxCCR1 CLLD bits control the update)"]
    #[inline(always)]
    pub fn tbclgrp_3(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TimerB interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&self) -> TBIFG_R {
        TBIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TimerB interrupt enable"]
    #[inline(always)]
    pub fn tbie(&self) -> TBIE_R {
        TBIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TimerB clear"]
    #[inline(always)]
    pub fn tbclr(&self) -> TBCLR_R {
        TBCLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - TimerB clock source select"]
    #[inline(always)]
    pub fn tbssel(&self) -> TBSSEL_R {
        TBSSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Counter length"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - TBxCLn group"]
    #[inline(always)]
    pub fn tbclgrp(&self) -> TBCLGRP_R {
        TBCLGRP_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerB interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&mut self) -> TBIFG_W {
        TBIFG_W { w: self }
    }
    #[doc = "Bit 1 - TimerB interrupt enable"]
    #[inline(always)]
    pub fn tbie(&mut self) -> TBIE_W {
        TBIE_W { w: self }
    }
    #[doc = "Bit 2 - TimerB clear"]
    #[inline(always)]
    pub fn tbclr(&mut self) -> TBCLR_W {
        TBCLR_W { w: self }
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:9 - TimerB clock source select"]
    #[inline(always)]
    pub fn tbssel(&mut self) -> TBSSEL_W {
        TBSSEL_W { w: self }
    }
    #[doc = "Bits 11:12 - Counter length"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CNTL_W {
        CNTL_W { w: self }
    }
    #[doc = "Bits 13:14 - TBxCLn group"]
    #[inline(always)]
    pub fn tbclgrp(&mut self) -> TBCLGRP_W {
        TBCLGRP_W { w: self }
    }
}
