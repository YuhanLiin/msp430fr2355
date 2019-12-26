#[doc = "Reader of register TB3CCTL5"]
pub type R = crate::R<u16, super::TB3CCTL5>;
#[doc = "Writer for register TB3CCTL5"]
pub type W = crate::W<u16, super::TB3CCTL5>;
#[doc = "Register TB3CCTL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::TB3CCTL5 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture/compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIFG_A {
    #[doc = "0: No interrupt pending"]
    CCIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    CCIFG_1 = 1,
}
impl From<CCIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CCIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCIFG`"]
pub type CCIFG_R = crate::R<bool, CCIFG_A>;
impl CCIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIFG_A {
        match self.bits {
            false => CCIFG_A::CCIFG_0,
            true => CCIFG_A::CCIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIFG_0`"]
    #[inline(always)]
    pub fn is_ccifg_0(&self) -> bool {
        *self == CCIFG_A::CCIFG_0
    }
    #[doc = "Checks if the value of the field is `CCIFG_1`"]
    #[inline(always)]
    pub fn is_ccifg_1(&self) -> bool {
        *self == CCIFG_A::CCIFG_1
    }
}
#[doc = "Write proxy for field `CCIFG`"]
pub struct CCIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ccifg_0(self) -> &'a mut W {
        self.variant(CCIFG_A::CCIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ccifg_1(self) -> &'a mut W {
        self.variant(CCIFG_A::CCIFG_1)
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
#[doc = "Capture overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COV_A {
    #[doc = "0: No capture overflow occurred"]
    COV_0 = 0,
    #[doc = "1: Capture overflow occurred"]
    COV_1 = 1,
}
impl From<COV_A> for bool {
    #[inline(always)]
    fn from(variant: COV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COV`"]
pub type COV_R = crate::R<bool, COV_A>;
impl COV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COV_A {
        match self.bits {
            false => COV_A::COV_0,
            true => COV_A::COV_1,
        }
    }
    #[doc = "Checks if the value of the field is `COV_0`"]
    #[inline(always)]
    pub fn is_cov_0(&self) -> bool {
        *self == COV_A::COV_0
    }
    #[doc = "Checks if the value of the field is `COV_1`"]
    #[inline(always)]
    pub fn is_cov_1(&self) -> bool {
        *self == COV_A::COV_1
    }
}
#[doc = "Write proxy for field `COV`"]
pub struct COV_W<'a> {
    w: &'a mut W,
}
impl<'a> COV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No capture overflow occurred"]
    #[inline(always)]
    pub fn cov_0(self) -> &'a mut W {
        self.variant(COV_A::COV_0)
    }
    #[doc = "Capture overflow occurred"]
    #[inline(always)]
    pub fn cov_1(self) -> &'a mut W {
        self.variant(COV_A::COV_1)
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
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT_A {
    #[doc = "0: Output low"]
    LOW = 0,
    #[doc = "1: Output high"]
    HIGH = 1,
}
impl From<OUT_A> for bool {
    #[inline(always)]
    fn from(variant: OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUT`"]
pub type OUT_R = crate::R<bool, OUT_A>;
impl OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_A {
        match self.bits {
            false => OUT_A::LOW,
            true => OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT_A::HIGH
    }
}
#[doc = "Write proxy for field `OUT`"]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT_A::LOW)
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT_A::HIGH)
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
#[doc = "Reader of field `CCI`"]
pub type CCI_R = crate::R<bool, bool>;
#[doc = "Capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
    #[doc = "0: Interrupt disabled"]
    CCIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CCIE_1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCIE`"]
pub type CCIE_R = crate::R<bool, CCIE_A>;
impl CCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::CCIE_0,
            true => CCIE_A::CCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIE_0`"]
    #[inline(always)]
    pub fn is_ccie_0(&self) -> bool {
        *self == CCIE_A::CCIE_0
    }
    #[doc = "Checks if the value of the field is `CCIE_1`"]
    #[inline(always)]
    pub fn is_ccie_1(&self) -> bool {
        *self == CCIE_A::CCIE_1
    }
}
#[doc = "Write proxy for field `CCIE`"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ccie_0(self) -> &'a mut W {
        self.variant(CCIE_A::CCIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ccie_1(self) -> &'a mut W {
        self.variant(CCIE_A::CCIE_1)
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
#[doc = "Output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMOD_A {
    #[doc = "0: OUT bit value"]
    OUTMOD_0 = 0,
    #[doc = "1: Set"]
    OUTMOD_1 = 1,
    #[doc = "2: Toggle/reset"]
    OUTMOD_2 = 2,
    #[doc = "3: Set/reset"]
    OUTMOD_3 = 3,
    #[doc = "4: Toggle"]
    OUTMOD_4 = 4,
    #[doc = "5: Reset"]
    OUTMOD_5 = 5,
    #[doc = "6: Toggle/set"]
    OUTMOD_6 = 6,
    #[doc = "7: Reset/set"]
    OUTMOD_7 = 7,
}
impl From<OUTMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTMOD`"]
pub type OUTMOD_R = crate::R<u8, OUTMOD_A>;
impl OUTMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMOD_A {
        match self.bits {
            0 => OUTMOD_A::OUTMOD_0,
            1 => OUTMOD_A::OUTMOD_1,
            2 => OUTMOD_A::OUTMOD_2,
            3 => OUTMOD_A::OUTMOD_3,
            4 => OUTMOD_A::OUTMOD_4,
            5 => OUTMOD_A::OUTMOD_5,
            6 => OUTMOD_A::OUTMOD_6,
            7 => OUTMOD_A::OUTMOD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTMOD_0`"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_0
    }
    #[doc = "Checks if the value of the field is `OUTMOD_1`"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_1
    }
    #[doc = "Checks if the value of the field is `OUTMOD_2`"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_2
    }
    #[doc = "Checks if the value of the field is `OUTMOD_3`"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_3
    }
    #[doc = "Checks if the value of the field is `OUTMOD_4`"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_4
    }
    #[doc = "Checks if the value of the field is `OUTMOD_5`"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_5
    }
    #[doc = "Checks if the value of the field is `OUTMOD_6`"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_6
    }
    #[doc = "Checks if the value of the field is `OUTMOD_7`"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_7
    }
}
#[doc = "Write proxy for field `OUTMOD`"]
pub struct OUTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "OUT bit value"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_0)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_1)
    }
    #[doc = "Toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_2)
    }
    #[doc = "Set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_3)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_4)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_5)
    }
    #[doc = "Toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_6)
    }
    #[doc = "Reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u16) & 0x07) << 5);
        self.w
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP_A {
    #[doc = "0: Compare mode"]
    COMPARE = 0,
    #[doc = "1: Capture mode"]
    CAPTURE = 1,
}
impl From<CAP_A> for bool {
    #[inline(always)]
    fn from(variant: CAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP`"]
pub type CAP_R = crate::R<bool, CAP_A>;
impl CAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP_A {
        match self.bits {
            false => CAP_A::COMPARE,
            true => CAP_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARE`"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == CAP_A::COMPARE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == CAP_A::CAPTURE
    }
}
#[doc = "Write proxy for field `CAP`"]
pub struct CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare mode"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut W {
        self.variant(CAP_A::COMPARE)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(CAP_A::CAPTURE)
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
#[doc = "Compare latch load\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLLD_A {
    #[doc = "0: TBxCLn loads on write to TBxCCRn"]
    CLLD_0 = 0,
    #[doc = "1: TBxCLn loads when TBxR counts to 0"]
    CLLD_1 = 1,
    #[doc = "2: TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode)."]
    CLLD_2 = 2,
    #[doc = "3: TBxCLn loads when TBxR counts to TBxCLn"]
    CLLD_3 = 3,
}
impl From<CLLD_A> for u8 {
    #[inline(always)]
    fn from(variant: CLLD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLLD`"]
pub type CLLD_R = crate::R<u8, CLLD_A>;
impl CLLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLLD_A {
        match self.bits {
            0 => CLLD_A::CLLD_0,
            1 => CLLD_A::CLLD_1,
            2 => CLLD_A::CLLD_2,
            3 => CLLD_A::CLLD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLLD_0`"]
    #[inline(always)]
    pub fn is_clld_0(&self) -> bool {
        *self == CLLD_A::CLLD_0
    }
    #[doc = "Checks if the value of the field is `CLLD_1`"]
    #[inline(always)]
    pub fn is_clld_1(&self) -> bool {
        *self == CLLD_A::CLLD_1
    }
    #[doc = "Checks if the value of the field is `CLLD_2`"]
    #[inline(always)]
    pub fn is_clld_2(&self) -> bool {
        *self == CLLD_A::CLLD_2
    }
    #[doc = "Checks if the value of the field is `CLLD_3`"]
    #[inline(always)]
    pub fn is_clld_3(&self) -> bool {
        *self == CLLD_A::CLLD_3
    }
}
#[doc = "Write proxy for field `CLLD`"]
pub struct CLLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLLD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TBxCLn loads on write to TBxCCRn"]
    #[inline(always)]
    pub fn clld_0(self) -> &'a mut W {
        self.variant(CLLD_A::CLLD_0)
    }
    #[doc = "TBxCLn loads when TBxR counts to 0"]
    #[inline(always)]
    pub fn clld_1(self) -> &'a mut W {
        self.variant(CLLD_A::CLLD_1)
    }
    #[doc = "TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode)."]
    #[inline(always)]
    pub fn clld_2(self) -> &'a mut W {
        self.variant(CLLD_A::CLLD_2)
    }
    #[doc = "TBxCLn loads when TBxR counts to TBxCLn"]
    #[inline(always)]
    pub fn clld_3(self) -> &'a mut W {
        self.variant(CLLD_A::CLLD_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u16) & 0x03) << 9);
        self.w
    }
}
#[doc = "Synchronize capture source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCS_A {
    #[doc = "0: Asynchronous capture"]
    ASYNC = 0,
    #[doc = "1: Synchronous capture"]
    SYNC = 1,
}
impl From<SCS_A> for bool {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<bool, SCS_A>;
impl SCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCS_A {
        match self.bits {
            false => SCS_A::ASYNC,
            true => SCS_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == SCS_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == SCS_A::SYNC
    }
}
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous capture"]
    #[inline(always)]
    pub fn async(self) -> &'a mut W {
        self.variant(SCS_A::ASYNC)
    }
    #[doc = "Synchronous capture"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(SCS_A::SYNC)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Capture/compare input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCIS_A {
    #[doc = "0: CCIxA"]
    CCIA = 0,
    #[doc = "1: CCIxB"]
    CCIB = 1,
    #[doc = "2: GND"]
    GND = 2,
    #[doc = "3: VCC"]
    VCC = 3,
}
impl From<CCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CCIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CCIS`"]
pub type CCIS_R = crate::R<u8, CCIS_A>;
impl CCIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIS_A {
        match self.bits {
            0 => CCIS_A::CCIA,
            1 => CCIS_A::CCIB,
            2 => CCIS_A::GND,
            3 => CCIS_A::VCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCIA`"]
    #[inline(always)]
    pub fn is_ccia(&self) -> bool {
        *self == CCIS_A::CCIA
    }
    #[doc = "Checks if the value of the field is `CCIB`"]
    #[inline(always)]
    pub fn is_ccib(&self) -> bool {
        *self == CCIS_A::CCIB
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == CCIS_A::GND
    }
    #[doc = "Checks if the value of the field is `VCC`"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == CCIS_A::VCC
    }
}
#[doc = "Write proxy for field `CCIS`"]
pub struct CCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCIxA"]
    #[inline(always)]
    pub fn ccia(self) -> &'a mut W {
        self.variant(CCIS_A::CCIA)
    }
    #[doc = "CCIxB"]
    #[inline(always)]
    pub fn ccib(self) -> &'a mut W {
        self.variant(CCIS_A::CCIB)
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(CCIS_A::GND)
    }
    #[doc = "VCC"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut W {
        self.variant(CCIS_A::VCC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: No capture"]
    NONE = 0,
    #[doc = "1: Capture on rising edge"]
    RISING = 1,
    #[doc = "2: Capture on falling edge"]
    FALLING = 2,
    #[doc = "3: Capture on both rising and falling edges"]
    BOTH = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::NONE,
            1 => CM_A::RISING,
            2 => CM_A::FALLING,
            3 => CM_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CM_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CM_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CM_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CM_A::BOTH
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No capture"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CM_A::NONE)
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CM_A::RISING)
    }
    #[doc = "Capture on falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CM_A::FALLING)
    }
    #[doc = "Capture on both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CM_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CCIFG_R {
        CCIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&self) -> COV_R {
        COV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/compare input"]
    #[inline(always)]
    pub fn cci(&self) -> CCI_R {
        CCI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&self) -> OUTMOD_R {
        OUTMOD_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Compare latch load"]
    #[inline(always)]
    pub fn clld(&self) -> CLLD_R {
        CLLD_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&self) -> CCIS_R {
        CCIS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&mut self) -> CCIFG_W {
        CCIFG_W { w: self }
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&mut self) -> COV_W {
        COV_W { w: self }
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&mut self) -> OUTMOD_W {
        OUTMOD_W { w: self }
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&mut self) -> CAP_W {
        CAP_W { w: self }
    }
    #[doc = "Bits 9:10 - Compare latch load"]
    #[inline(always)]
    pub fn clld(&mut self) -> CLLD_W {
        CLLD_W { w: self }
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&mut self) -> CCIS_W {
        CCIS_W { w: self }
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
}
