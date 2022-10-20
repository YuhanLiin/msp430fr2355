#[doc = "Register `UCB0CTLW0` reader"]
pub struct R(crate::R<UCB0CTLW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0CTLW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0CTLW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0CTLW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0CTLW0` writer"]
pub struct W(crate::W<UCB0CTLW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0CTLW0_SPEC>;
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
impl From<crate::W<UCB0CTLW0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0CTLW0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UCSWRST_R = crate::BitReader<UCSWRST_A>;
#[doc = "Software reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSWRST_A {
    #[doc = "0: Disabled. eUSCI_B reset released for operation"]
    DISABLE = 0,
    #[doc = "1: Enabled. eUSCI_B logic held in reset state"]
    ENABLE = 1,
}
impl From<UCSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: UCSWRST_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSWRST_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UCSWRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCSWRST_A, O>;
impl<'a, const O: u8> UCSWRST_W<'a, O> {
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
}
#[doc = "Field `UCTXSTT` reader - Transmit START condition in master mode"]
pub type UCTXSTT_R = crate::BitReader<UCTXSTT_A>;
#[doc = "Transmit START condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXSTT_A {
    #[doc = "0: Do not generate START condition"]
    UCTXSTT_0 = 0,
    #[doc = "1: Generate START condition"]
    UCTXSTT_1 = 1,
}
impl From<UCTXSTT_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXSTT_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXSTT_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCTXSTT` writer - Transmit START condition in master mode"]
pub type UCTXSTT_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCTXSTT_A, O>;
impl<'a, const O: u8> UCTXSTT_W<'a, O> {
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
}
#[doc = "Field `UCTXSTP` reader - Transmit STOP condition in master mode"]
pub type UCTXSTP_R = crate::BitReader<UCTXSTP_A>;
#[doc = "Transmit STOP condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXSTP_A {
    #[doc = "0: No STOP generated"]
    UCTXSTP_0 = 0,
    #[doc = "1: Generate STOP"]
    UCTXSTP_1 = 1,
}
impl From<UCTXSTP_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXSTP_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCTXSTP` writer - Transmit STOP condition in master mode"]
pub type UCTXSTP_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCTXSTP_A, O>;
impl<'a, const O: u8> UCTXSTP_W<'a, O> {
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
}
#[doc = "Field `UCTXNACK` reader - Transmit a NACK"]
pub type UCTXNACK_R = crate::BitReader<UCTXNACK_A>;
#[doc = "Transmit a NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXNACK_A {
    #[doc = "0: Acknowledge normally"]
    UCTXNACK_0 = 0,
    #[doc = "1: Generate NACK"]
    UCTXNACK_1 = 1,
}
impl From<UCTXNACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXNACK_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXNACK_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCTXNACK` writer - Transmit a NACK"]
pub type UCTXNACK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCTXNACK_A, O>;
impl<'a, const O: u8> UCTXNACK_W<'a, O> {
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
}
#[doc = "Field `UCTR` reader - Transmitter/receiver"]
pub type UCTR_R = crate::BitReader<UCTR_A>;
#[doc = "Transmitter/receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTR_A {
    #[doc = "0: Receiver"]
    RX = 0,
    #[doc = "1: Transmitter"]
    TX = 1,
}
impl From<UCTR_A> for bool {
    #[inline(always)]
    fn from(variant: UCTR_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCTR` writer - Transmitter/receiver"]
pub type UCTR_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCTR_A, O>;
impl<'a, const O: u8> UCTR_W<'a, O> {
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
}
#[doc = "Field `UCTXACK` reader - Transmit ACK condition in slave mode"]
pub type UCTXACK_R = crate::BitReader<UCTXACK_A>;
#[doc = "Transmit ACK condition in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXACK_A {
    #[doc = "0: Do not acknowledge the slave address"]
    UCTXACK_0 = 0,
    #[doc = "1: Acknowledge the slave address"]
    UCTXACK_1 = 1,
}
impl From<UCTXACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXACK_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXACK_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCTXACK` writer - Transmit ACK condition in slave mode"]
pub type UCTXACK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCTXACK_A, O>;
impl<'a, const O: u8> UCTXACK_W<'a, O> {
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
}
#[doc = "Field `UCSSEL` reader - eUSCI_B clock source select"]
pub type UCSSEL_R = crate::FieldReader<u8, UCSSEL_A>;
#[doc = "eUSCI_B clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSSEL_A {
    #[doc = "0: UCLKI"]
    UCLKI = 0,
    #[doc = "1: ACLK"]
    ACLK = 1,
    #[doc = "2: SMCLK"]
    SMCLK = 2,
    #[doc = "3: SMCLK"]
    UCSSEL_3 = 3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        variant as _
    }
}
impl UCSSEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCSSEL` writer - eUSCI_B clock source select"]
pub type UCSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCB0CTLW0_SPEC, u8, UCSSEL_A, 2, O>;
impl<'a, const O: u8> UCSSEL_W<'a, O> {
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
}
#[doc = "Field `UCSYNC` reader - Synchronous mode enable"]
pub type UCSYNC_R = crate::BitReader<UCSYNC_A>;
#[doc = "Synchronous mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSYNC_A {
    #[doc = "0: Asynchronous mode"]
    ASYNC = 0,
    #[doc = "1: Synchronous mode"]
    SYNC = 1,
}
impl From<UCSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: UCSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSYNC_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCSYNC` writer - Synchronous mode enable"]
pub type UCSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCSYNC_A, O>;
impl<'a, const O: u8> UCSYNC_W<'a, O> {
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(UCSYNC_A::ASYNC)
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(UCSYNC_A::SYNC)
    }
}
#[doc = "Field `UCMODE` reader - eUSCI_B mode"]
pub type UCMODE_R = crate::FieldReader<u8, UCMODE_A>;
#[doc = "eUSCI_B mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: 3-pin SPI"]
    UCMODE_0 = 0,
    #[doc = "1: 4-pin SPI (master or slave enabled if STE = 1)"]
    UCMODE_1 = 1,
    #[doc = "2: 4-pin SPI (master or slave enabled if STE = 0)"]
    UCMODE_2 = 2,
    #[doc = "3: I2C mode"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
impl UCMODE_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCMODE` writer - eUSCI_B mode"]
pub type UCMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCB0CTLW0_SPEC, u8, UCMODE_A, 2, O>;
impl<'a, const O: u8> UCMODE_W<'a, O> {
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
}
#[doc = "Field `UCMST` reader - Master mode select"]
pub type UCMST_R = crate::BitReader<UCMST_A>;
#[doc = "Master mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCMST_A {
    #[doc = "0: Slave mode"]
    SLAVE = 0,
    #[doc = "1: Master mode"]
    MASTER = 1,
}
impl From<UCMST_A> for bool {
    #[inline(always)]
    fn from(variant: UCMST_A) -> Self {
        variant as u8 != 0
    }
}
impl UCMST_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCMST` writer - Master mode select"]
pub type UCMST_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCMST_A, O>;
impl<'a, const O: u8> UCMST_W<'a, O> {
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
}
#[doc = "Field `UCMM` reader - Multi-master environment select"]
pub type UCMM_R = crate::BitReader<UCMM_A>;
#[doc = "Multi-master environment select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCMM_A {
    #[doc = "0: Single master environment. There is no other master in the system. The address compare unit is disabled."]
    SINGLE = 0,
    #[doc = "1: Multi-master environment"]
    MULTI = 1,
}
impl From<UCMM_A> for bool {
    #[inline(always)]
    fn from(variant: UCMM_A) -> Self {
        variant as u8 != 0
    }
}
impl UCMM_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCMM` writer - Multi-master environment select"]
pub type UCMM_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCMM_A, O>;
impl<'a, const O: u8> UCMM_W<'a, O> {
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
}
#[doc = "Field `UCSLA10` reader - Slave addressing mode select"]
pub type UCSLA10_R = crate::BitReader<UCSLA10_A>;
#[doc = "Slave addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSLA10_A {
    #[doc = "0: Address slave with 7-bit address"]
    _7BIT = 0,
    #[doc = "1: Address slave with 10-bit address"]
    _10BIT = 1,
}
impl From<UCSLA10_A> for bool {
    #[inline(always)]
    fn from(variant: UCSLA10_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSLA10_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCSLA10` writer - Slave addressing mode select"]
pub type UCSLA10_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCSLA10_A, O>;
impl<'a, const O: u8> UCSLA10_W<'a, O> {
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
}
#[doc = "Field `UCA10` reader - Own addressing mode select"]
pub type UCA10_R = crate::BitReader<UCA10_A>;
#[doc = "Own addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCA10_A {
    #[doc = "0: Own address is a 7-bit address"]
    UCA10_0 = 0,
    #[doc = "1: Own address is a 10-bit address"]
    UCA10_1 = 1,
}
impl From<UCA10_A> for bool {
    #[inline(always)]
    fn from(variant: UCA10_A) -> Self {
        variant as u8 != 0
    }
}
impl UCA10_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `UCA10` writer - Own addressing mode select"]
pub type UCA10_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPEC, UCA10_A, O>;
impl<'a, const O: u8> UCA10_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&self) -> UCTXSTT_R {
        UCTXSTT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&self) -> UCTXSTP_R {
        UCTXSTP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&self) -> UCTXNACK_R {
        UCTXNACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&self) -> UCTR_R {
        UCTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&self) -> UCTXACK_R {
        UCTXACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UCSSEL_R {
        UCSSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&self) -> UCMM_R {
        UCMM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&self) -> UCSLA10_R {
        UCSLA10_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&self) -> UCA10_R {
        UCA10_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W<0> {
        UCSWRST_W::new(self)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&mut self) -> UCTXSTT_W<1> {
        UCTXSTT_W::new(self)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&mut self) -> UCTXSTP_W<2> {
        UCTXSTP_W::new(self)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&mut self) -> UCTXNACK_W<3> {
        UCTXNACK_W::new(self)
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&mut self) -> UCTR_W<4> {
        UCTR_W::new(self)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&mut self) -> UCTXACK_W<5> {
        UCTXACK_W::new(self)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UCSSEL_W<6> {
        UCSSEL_W::new(self)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W<8> {
        UCSYNC_W::new(self)
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W<9> {
        UCMODE_W::new(self)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W<11> {
        UCMST_W::new(self)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UCMM_W<13> {
        UCMM_W::new(self)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> UCSLA10_W<14> {
        UCSLA10_W::new(self)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&mut self) -> UCA10_W<15> {
        UCA10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctlw0](index.html) module"]
pub struct UCB0CTLW0_SPEC;
impl crate::RegisterSpec for UCB0CTLW0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0ctlw0::R](R) reader structure"]
impl crate::Readable for UCB0CTLW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ctlw0::W](W) writer structure"]
impl crate::Writable for UCB0CTLW0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0CTLW0 to value 0"]
impl crate::Resettable for UCB0CTLW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
