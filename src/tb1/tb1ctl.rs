#[doc = "Register `TB1CTL` reader"]
pub struct R(crate::R<TB1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TB1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TB1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TB1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TB1CTL` writer"]
pub struct W(crate::W<TB1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TB1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TB1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TB1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBIFG` reader - TimerB interrupt flag"]
pub type TBIFG_R = crate::BitReader<TBIFG_A>;
#[doc = "TimerB interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBIFG_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `TBIFG` writer - TimerB interrupt flag"]
pub type TBIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TB1CTL_SPEC, TBIFG_A, O>;
impl<'a, const O: u8> TBIFG_W<'a, O> {
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
}
#[doc = "Field `TBIE` reader - TimerB interrupt enable"]
pub type TBIE_R = crate::BitReader<TBIE_A>;
#[doc = "TimerB interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBIE_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `TBIE` writer - TimerB interrupt enable"]
pub type TBIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TB1CTL_SPEC, TBIE_A, O>;
impl<'a, const O: u8> TBIE_W<'a, O> {
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
}
#[doc = "Field `TBCLR` reader - TimerB clear"]
pub type TBCLR_R = crate::BitReader<bool>;
#[doc = "Field `TBCLR` writer - TimerB clear"]
pub type TBCLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, TB1CTL_SPEC, bool, O>;
#[doc = "Field `MC` reader - Mode control"]
pub type MC_R = crate::FieldReader<u8, MC_A>;
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MC_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `MC` writer - Mode control"]
pub type MC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TB1CTL_SPEC, u8, MC_A, 2, O>;
impl<'a, const O: u8> MC_W<'a, O> {
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
}
#[doc = "Field `ID` reader - Input divider"]
pub type ID_R = crate::FieldReader<u8, ID_A>;
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ID_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ID` writer - Input divider"]
pub type ID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TB1CTL_SPEC, u8, ID_A, 2, O>;
impl<'a, const O: u8> ID_W<'a, O> {
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
}
#[doc = "Field `TBSSEL` reader - TimerB clock source select"]
pub type TBSSEL_R = crate::FieldReader<u8, TBSSEL_A>;
#[doc = "TimerB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBSSEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `TBSSEL` writer - TimerB clock source select"]
pub type TBSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TB1CTL_SPEC, u8, TBSSEL_A, 2, O>;
impl<'a, const O: u8> TBSSEL_W<'a, O> {
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
}
#[doc = "Field `CNTL` reader - Counter length"]
pub type CNTL_R = crate::FieldReader<u8, CNTL_A>;
#[doc = "Counter length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CNTL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `CNTL` writer - Counter length"]
pub type CNTL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TB1CTL_SPEC, u8, CNTL_A, 2, O>;
impl<'a, const O: u8> CNTL_W<'a, O> {
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
}
#[doc = "Field `TBCLGRP` reader - TBxCLn group"]
pub type TBCLGRP_R = crate::FieldReader<u8, TBCLGRP_A>;
#[doc = "TBxCLn group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBCLGRP_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `TBCLGRP` writer - TBxCLn group"]
pub type TBCLGRP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TB1CTL_SPEC, u8, TBCLGRP_A, 2, O>;
impl<'a, const O: u8> TBCLGRP_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - TimerB interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&self) -> TBIFG_R {
        TBIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TimerB interrupt enable"]
    #[inline(always)]
    pub fn tbie(&self) -> TBIE_R {
        TBIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TimerB clear"]
    #[inline(always)]
    pub fn tbclr(&self) -> TBCLR_R {
        TBCLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TimerB clock source select"]
    #[inline(always)]
    pub fn tbssel(&self) -> TBSSEL_R {
        TBSSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Counter length"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - TBxCLn group"]
    #[inline(always)]
    pub fn tbclgrp(&self) -> TBCLGRP_R {
        TBCLGRP_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerB interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&mut self) -> TBIFG_W<0> {
        TBIFG_W::new(self)
    }
    #[doc = "Bit 1 - TimerB interrupt enable"]
    #[inline(always)]
    pub fn tbie(&mut self) -> TBIE_W<1> {
        TBIE_W::new(self)
    }
    #[doc = "Bit 2 - TimerB clear"]
    #[inline(always)]
    pub fn tbclr(&mut self) -> TBCLR_W<2> {
        TBCLR_W::new(self)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W<4> {
        MC_W::new(self)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W<6> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:9 - TimerB clock source select"]
    #[inline(always)]
    pub fn tbssel(&mut self) -> TBSSEL_W<8> {
        TBSSEL_W::new(self)
    }
    #[doc = "Bits 11:12 - Counter length"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CNTL_W<11> {
        CNTL_W::new(self)
    }
    #[doc = "Bits 13:14 - TBxCLn group"]
    #[inline(always)]
    pub fn tbclgrp(&mut self) -> TBCLGRP_W<13> {
        TBCLGRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer_B Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb1ctl](index.html) module"]
pub struct TB1CTL_SPEC;
impl crate::RegisterSpec for TB1CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tb1ctl::R](R) reader structure"]
impl crate::Readable for TB1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tb1ctl::W](W) writer structure"]
impl crate::Writable for TB1CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TB1CTL to value 0"]
impl crate::Resettable for TB1CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
