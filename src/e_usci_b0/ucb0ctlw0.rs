#[doc = "Reader of register UCB0CTLW0"]
pub type R = crate::R<u16, super::UCB0CTLW0>;
#[doc = "Writer for register UCB0CTLW0"]
pub type W = crate::W<u16, super::UCB0CTLW0>;
#[doc = "Register UCB0CTLW0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0CTLW0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSWRST_A {
    #[doc = "0: Disabled. eUSCI_B reset released for operation"]
    DISABLE,
    #[doc = "1: Enabled. eUSCI_B logic held in reset state"]
    ENABLE,
}
impl From<UCSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: UCSWRST_A) -> Self {
        match variant {
            UCSWRST_A::DISABLE => false,
            UCSWRST_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `UCSWRST`"]
pub type UCSWRST_R = crate::R<bool, UCSWRST_A>;
impl UCSWRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSWRST_A {
        match self.bits {
            false => UCSWRST_A::DISABLE,
            true => UCSWRST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UCSWRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UCSWRST_A::ENABLE
    }
}
#[doc = "Write proxy for field `UCSWRST`"]
pub struct UCSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSWRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. eUSCI_B reset released for operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UCSWRST_A::DISABLE)
    }
    #[doc = "Enabled. eUSCI_B logic held in reset state"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UCSWRST_A::ENABLE)
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
#[doc = "Transmit START condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXSTT_A {
    #[doc = "0: Do not generate START condition"]
    UCTXSTT_0,
    #[doc = "1: Generate START condition"]
    UCTXSTT_1,
}
impl From<UCTXSTT_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXSTT_A) -> Self {
        match variant {
            UCTXSTT_A::UCTXSTT_0 => false,
            UCTXSTT_A::UCTXSTT_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXSTT`"]
pub type UCTXSTT_R = crate::R<bool, UCTXSTT_A>;
impl UCTXSTT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXSTT_A {
        match self.bits {
            false => UCTXSTT_A::UCTXSTT_0,
            true => UCTXSTT_A::UCTXSTT_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXSTT_0`"]
    #[inline(always)]
    pub fn is_uctxstt_0(&self) -> bool {
        *self == UCTXSTT_A::UCTXSTT_0
    }
    #[doc = "Checks if the value of the field is `UCTXSTT_1`"]
    #[inline(always)]
    pub fn is_uctxstt_1(&self) -> bool {
        *self == UCTXSTT_A::UCTXSTT_1
    }
}
#[doc = "Write proxy for field `UCTXSTT`"]
pub struct UCTXSTT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXSTT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXSTT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not generate START condition"]
    #[inline(always)]
    pub fn uctxstt_0(self) -> &'a mut W {
        self.variant(UCTXSTT_A::UCTXSTT_0)
    }
    #[doc = "Generate START condition"]
    #[inline(always)]
    pub fn uctxstt_1(self) -> &'a mut W {
        self.variant(UCTXSTT_A::UCTXSTT_1)
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
#[doc = "Transmit STOP condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXSTP_A {
    #[doc = "0: No STOP generated"]
    UCTXSTP_0,
    #[doc = "1: Generate STOP"]
    UCTXSTP_1,
}
impl From<UCTXSTP_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXSTP_A) -> Self {
        match variant {
            UCTXSTP_A::UCTXSTP_0 => false,
            UCTXSTP_A::UCTXSTP_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXSTP`"]
pub type UCTXSTP_R = crate::R<bool, UCTXSTP_A>;
impl UCTXSTP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXSTP_A {
        match self.bits {
            false => UCTXSTP_A::UCTXSTP_0,
            true => UCTXSTP_A::UCTXSTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXSTP_0`"]
    #[inline(always)]
    pub fn is_uctxstp_0(&self) -> bool {
        *self == UCTXSTP_A::UCTXSTP_0
    }
    #[doc = "Checks if the value of the field is `UCTXSTP_1`"]
    #[inline(always)]
    pub fn is_uctxstp_1(&self) -> bool {
        *self == UCTXSTP_A::UCTXSTP_1
    }
}
#[doc = "Write proxy for field `UCTXSTP`"]
pub struct UCTXSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXSTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXSTP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No STOP generated"]
    #[inline(always)]
    pub fn uctxstp_0(self) -> &'a mut W {
        self.variant(UCTXSTP_A::UCTXSTP_0)
    }
    #[doc = "Generate STOP"]
    #[inline(always)]
    pub fn uctxstp_1(self) -> &'a mut W {
        self.variant(UCTXSTP_A::UCTXSTP_1)
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
#[doc = "Transmit a NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXNACK_A {
    #[doc = "0: Acknowledge normally"]
    UCTXNACK_0,
    #[doc = "1: Generate NACK"]
    UCTXNACK_1,
}
impl From<UCTXNACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXNACK_A) -> Self {
        match variant {
            UCTXNACK_A::UCTXNACK_0 => false,
            UCTXNACK_A::UCTXNACK_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXNACK`"]
pub type UCTXNACK_R = crate::R<bool, UCTXNACK_A>;
impl UCTXNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXNACK_A {
        match self.bits {
            false => UCTXNACK_A::UCTXNACK_0,
            true => UCTXNACK_A::UCTXNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXNACK_0`"]
    #[inline(always)]
    pub fn is_uctxnack_0(&self) -> bool {
        *self == UCTXNACK_A::UCTXNACK_0
    }
    #[doc = "Checks if the value of the field is `UCTXNACK_1`"]
    #[inline(always)]
    pub fn is_uctxnack_1(&self) -> bool {
        *self == UCTXNACK_A::UCTXNACK_1
    }
}
#[doc = "Write proxy for field `UCTXNACK`"]
pub struct UCTXNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Acknowledge normally"]
    #[inline(always)]
    pub fn uctxnack_0(self) -> &'a mut W {
        self.variant(UCTXNACK_A::UCTXNACK_0)
    }
    #[doc = "Generate NACK"]
    #[inline(always)]
    pub fn uctxnack_1(self) -> &'a mut W {
        self.variant(UCTXNACK_A::UCTXNACK_1)
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
#[doc = "Transmitter/receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTR_A {
    #[doc = "0: Receiver"]
    RX,
    #[doc = "1: Transmitter"]
    TX,
}
impl From<UCTR_A> for bool {
    #[inline(always)]
    fn from(variant: UCTR_A) -> Self {
        match variant {
            UCTR_A::RX => false,
            UCTR_A::TX => true,
        }
    }
}
#[doc = "Reader of field `UCTR`"]
pub type UCTR_R = crate::R<bool, UCTR_A>;
impl UCTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTR_A {
        match self.bits {
            false => UCTR_A::RX,
            true => UCTR_A::TX,
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == UCTR_A::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == UCTR_A::TX
    }
}
#[doc = "Write proxy for field `UCTR`"]
pub struct UCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut W {
        self.variant(UCTR_A::RX)
    }
    #[doc = "Transmitter"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(UCTR_A::TX)
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
#[doc = "Transmit ACK condition in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXACK_A {
    #[doc = "0: Do not acknowledge the slave address"]
    UCTXACK_0,
    #[doc = "1: Acknowledge the slave address"]
    UCTXACK_1,
}
impl From<UCTXACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXACK_A) -> Self {
        match variant {
            UCTXACK_A::UCTXACK_0 => false,
            UCTXACK_A::UCTXACK_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXACK`"]
pub type UCTXACK_R = crate::R<bool, UCTXACK_A>;
impl UCTXACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXACK_A {
        match self.bits {
            false => UCTXACK_A::UCTXACK_0,
            true => UCTXACK_A::UCTXACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXACK_0`"]
    #[inline(always)]
    pub fn is_uctxack_0(&self) -> bool {
        *self == UCTXACK_A::UCTXACK_0
    }
    #[doc = "Checks if the value of the field is `UCTXACK_1`"]
    #[inline(always)]
    pub fn is_uctxack_1(&self) -> bool {
        *self == UCTXACK_A::UCTXACK_1
    }
}
#[doc = "Write proxy for field `UCTXACK`"]
pub struct UCTXACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not acknowledge the slave address"]
    #[inline(always)]
    pub fn uctxack_0(self) -> &'a mut W {
        self.variant(UCTXACK_A::UCTXACK_0)
    }
    #[doc = "Acknowledge the slave address"]
    #[inline(always)]
    pub fn uctxack_1(self) -> &'a mut W {
        self.variant(UCTXACK_A::UCTXACK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "eUSCI_B clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSSEL_A {
    #[doc = "0: UCLKI"]
    UCLKI,
    #[doc = "1: ACLK"]
    ACLK,
    #[doc = "2: SMCLK"]
    SMCLK,
    #[doc = "3: SMCLK"]
    UCSSEL_3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        match variant {
            UCSSEL_A::UCLKI => 0,
            UCSSEL_A::ACLK => 1,
            UCSSEL_A::SMCLK => 2,
            UCSSEL_A::UCSSEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `UCSSEL`"]
pub type UCSSEL_R = crate::R<u8, UCSSEL_A>;
impl UCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSSEL_A {
        match self.bits {
            0 => UCSSEL_A::UCLKI,
            1 => UCSSEL_A::ACLK,
            2 => UCSSEL_A::SMCLK,
            3 => UCSSEL_A::UCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCLKI`"]
    #[inline(always)]
    pub fn is_uclki(&self) -> bool {
        *self == UCSSEL_A::UCLKI
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == UCSSEL_A::ACLK
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == UCSSEL_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `UCSSEL_3`"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_3
    }
}
#[doc = "Write proxy for field `UCSSEL`"]
pub struct UCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "UCLKI"]
    #[inline(always)]
    pub fn uclki(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCLKI)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(UCSSEL_A::ACLK)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(UCSSEL_A::SMCLK)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Synchronous mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSYNC_A {
    #[doc = "0: Asynchronous mode"]
    ASYNC,
    #[doc = "1: Synchronous mode"]
    SYNC,
}
impl From<UCSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: UCSYNC_A) -> Self {
        match variant {
            UCSYNC_A::ASYNC => false,
            UCSYNC_A::SYNC => true,
        }
    }
}
#[doc = "Reader of field `UCSYNC`"]
pub type UCSYNC_R = crate::R<bool, UCSYNC_A>;
impl UCSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSYNC_A {
        match self.bits {
            false => UCSYNC_A::ASYNC,
            true => UCSYNC_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == UCSYNC_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == UCSYNC_A::SYNC
    }
}
#[doc = "Write proxy for field `UCSYNC`"]
pub struct UCSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn async(self) -> &'a mut W {
        self.variant(UCSYNC_A::ASYNC)
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(UCSYNC_A::SYNC)
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
#[doc = "eUSCI_B mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMODE_A {
    #[doc = "0: 3-pin SPI"]
    UCMODE_0,
    #[doc = "1: 4-pin SPI (master or slave enabled if STE = 1)"]
    UCMODE_1,
    #[doc = "2: 4-pin SPI (master or slave enabled if STE = 0)"]
    UCMODE_2,
    #[doc = "3: I2C mode"]
    UCMODE_3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        match variant {
            UCMODE_A::UCMODE_0 => 0,
            UCMODE_A::UCMODE_1 => 1,
            UCMODE_A::UCMODE_2 => 2,
            UCMODE_A::UCMODE_3 => 3,
        }
    }
}
#[doc = "Reader of field `UCMODE`"]
pub type UCMODE_R = crate::R<u8, UCMODE_A>;
impl UCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCMODE_0`"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == UCMODE_A::UCMODE_0
    }
    #[doc = "Checks if the value of the field is `UCMODE_1`"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == UCMODE_A::UCMODE_1
    }
    #[doc = "Checks if the value of the field is `UCMODE_2`"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == UCMODE_A::UCMODE_2
    }
    #[doc = "Checks if the value of the field is `UCMODE_3`"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == UCMODE_A::UCMODE_3
    }
}
#[doc = "Write proxy for field `UCMODE`"]
pub struct UCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3-pin SPI"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 1)"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 0)"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u16) & 0x03) << 9);
        self.w
    }
}
#[doc = "Master mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMST_A {
    #[doc = "0: Slave mode"]
    SLAVE,
    #[doc = "1: Master mode"]
    MASTER,
}
impl From<UCMST_A> for bool {
    #[inline(always)]
    fn from(variant: UCMST_A) -> Self {
        match variant {
            UCMST_A::SLAVE => false,
            UCMST_A::MASTER => true,
        }
    }
}
#[doc = "Reader of field `UCMST`"]
pub type UCMST_R = crate::R<bool, UCMST_A>;
impl UCMST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMST_A {
        match self.bits {
            false => UCMST_A::SLAVE,
            true => UCMST_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == UCMST_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == UCMST_A::MASTER
    }
}
#[doc = "Write proxy for field `UCMST`"]
pub struct UCMST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(UCMST_A::SLAVE)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(UCMST_A::MASTER)
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
#[doc = "Multi-master environment select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMM_A {
    #[doc = "0: Single master environment. There is no other master in the system. The address compare unit is disabled."]
    SINGLE,
    #[doc = "1: Multi-master environment"]
    MULTI,
}
impl From<UCMM_A> for bool {
    #[inline(always)]
    fn from(variant: UCMM_A) -> Self {
        match variant {
            UCMM_A::SINGLE => false,
            UCMM_A::MULTI => true,
        }
    }
}
#[doc = "Reader of field `UCMM`"]
pub type UCMM_R = crate::R<bool, UCMM_A>;
impl UCMM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMM_A {
        match self.bits {
            false => UCMM_A::SINGLE,
            true => UCMM_A::MULTI,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == UCMM_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTI`"]
    #[inline(always)]
    pub fn is_multi(&self) -> bool {
        *self == UCMM_A::MULTI
    }
}
#[doc = "Write proxy for field `UCMM`"]
pub struct UCMM_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single master environment. There is no other master in the system. The address compare unit is disabled."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(UCMM_A::SINGLE)
    }
    #[doc = "Multi-master environment"]
    #[inline(always)]
    pub fn multi(self) -> &'a mut W {
        self.variant(UCMM_A::MULTI)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Slave addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSLA10_A {
    #[doc = "0: Address slave with 7-bit address"]
    _7BIT,
    #[doc = "1: Address slave with 10-bit address"]
    _10BIT,
}
impl From<UCSLA10_A> for bool {
    #[inline(always)]
    fn from(variant: UCSLA10_A) -> Self {
        match variant {
            UCSLA10_A::_7BIT => false,
            UCSLA10_A::_10BIT => true,
        }
    }
}
#[doc = "Reader of field `UCSLA10`"]
pub type UCSLA10_R = crate::R<bool, UCSLA10_A>;
impl UCSLA10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSLA10_A {
        match self.bits {
            false => UCSLA10_A::_7BIT,
            true => UCSLA10_A::_10BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_7BIT`"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == UCSLA10_A::_7BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == UCSLA10_A::_10BIT
    }
}
#[doc = "Write proxy for field `UCSLA10`"]
pub struct UCSLA10_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSLA10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSLA10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Address slave with 7-bit address"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut W {
        self.variant(UCSLA10_A::_7BIT)
    }
    #[doc = "Address slave with 10-bit address"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(UCSLA10_A::_10BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Own addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCA10_A {
    #[doc = "0: Own address is a 7-bit address"]
    UCA10_0,
    #[doc = "1: Own address is a 10-bit address"]
    UCA10_1,
}
impl From<UCA10_A> for bool {
    #[inline(always)]
    fn from(variant: UCA10_A) -> Self {
        match variant {
            UCA10_A::UCA10_0 => false,
            UCA10_A::UCA10_1 => true,
        }
    }
}
#[doc = "Reader of field `UCA10`"]
pub type UCA10_R = crate::R<bool, UCA10_A>;
impl UCA10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCA10_A {
        match self.bits {
            false => UCA10_A::UCA10_0,
            true => UCA10_A::UCA10_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCA10_0`"]
    #[inline(always)]
    pub fn is_uca10_0(&self) -> bool {
        *self == UCA10_A::UCA10_0
    }
    #[doc = "Checks if the value of the field is `UCA10_1`"]
    #[inline(always)]
    pub fn is_uca10_1(&self) -> bool {
        *self == UCA10_A::UCA10_1
    }
}
#[doc = "Write proxy for field `UCA10`"]
pub struct UCA10_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCA10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Own address is a 7-bit address"]
    #[inline(always)]
    pub fn uca10_0(self) -> &'a mut W {
        self.variant(UCA10_A::UCA10_0)
    }
    #[doc = "Own address is a 10-bit address"]
    #[inline(always)]
    pub fn uca10_1(self) -> &'a mut W {
        self.variant(UCA10_A::UCA10_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&self) -> UCTXSTT_R {
        UCTXSTT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&self) -> UCTXSTP_R {
        UCTXSTP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&self) -> UCTXNACK_R {
        UCTXNACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&self) -> UCTR_R {
        UCTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&self) -> UCTXACK_R {
        UCTXACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UCSSEL_R {
        UCSSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&self) -> UCMM_R {
        UCMM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&self) -> UCSLA10_R {
        UCSLA10_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&self) -> UCA10_R {
        UCA10_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W {
        UCSWRST_W { w: self }
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&mut self) -> UCTXSTT_W {
        UCTXSTT_W { w: self }
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&mut self) -> UCTXSTP_W {
        UCTXSTP_W { w: self }
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&mut self) -> UCTXNACK_W {
        UCTXNACK_W { w: self }
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&mut self) -> UCTR_W {
        UCTR_W { w: self }
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&mut self) -> UCTXACK_W {
        UCTXACK_W { w: self }
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UCSSEL_W {
        UCSSEL_W { w: self }
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W {
        UCSYNC_W { w: self }
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W {
        UCMODE_W { w: self }
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W {
        UCMST_W { w: self }
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UCMM_W {
        UCMM_W { w: self }
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> UCSLA10_W {
        UCSLA10_W { w: self }
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&mut self) -> UCA10_W {
        UCA10_W { w: self }
    }
}
