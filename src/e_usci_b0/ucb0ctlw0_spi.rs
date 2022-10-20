#[doc = "Register `UCB0CTLW0_SPI` reader"]
pub struct R(crate::R<UCB0CTLW0_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0CTLW0_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0CTLW0_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0CTLW0_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0CTLW0_SPI` writer"]
pub struct W(crate::W<UCB0CTLW0_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0CTLW0_SPI_SPEC>;
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
impl From<crate::W<UCB0CTLW0_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0CTLW0_SPI_SPEC>) -> Self {
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
pub type UCSWRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UCSWRST_A, O>;
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
#[doc = "Field `UCSTEM` reader - STE mode select in master mode."]
pub type UCSTEM_R = crate::BitReader<UCSTEM_A>;
#[doc = "STE mode select in master mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSTEM_A {
    #[doc = "0: STE pin is used to prevent conflicts with other masters"]
    UCSTEM_0 = 0,
    #[doc = "1: STE pin is used to generate the enable signal for a 4-wire slave"]
    UCSTEM_1 = 1,
}
impl From<UCSTEM_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTEM_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSTEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTEM_A {
        match self.bits {
            false => UCSTEM_A::UCSTEM_0,
            true => UCSTEM_A::UCSTEM_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTEM_0`"]
    #[inline(always)]
    pub fn is_ucstem_0(&self) -> bool {
        *self == UCSTEM_A::UCSTEM_0
    }
    #[doc = "Checks if the value of the field is `UCSTEM_1`"]
    #[inline(always)]
    pub fn is_ucstem_1(&self) -> bool {
        *self == UCSTEM_A::UCSTEM_1
    }
}
#[doc = "Field `UCSTEM` writer - STE mode select in master mode."]
pub type UCSTEM_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UCSTEM_A, O>;
impl<'a, const O: u8> UCSTEM_W<'a, O> {
    #[doc = "STE pin is used to prevent conflicts with other masters"]
    #[inline(always)]
    pub fn ucstem_0(self) -> &'a mut W {
        self.variant(UCSTEM_A::UCSTEM_0)
    }
    #[doc = "STE pin is used to generate the enable signal for a 4-wire slave"]
    #[inline(always)]
    pub fn ucstem_1(self) -> &'a mut W {
        self.variant(UCSTEM_A::UCSTEM_1)
    }
}
#[doc = "Field `UCSSEL` reader - eUSCI_B clock source select"]
pub type UCSSEL_R = crate::FieldReader<u8, UCSSEL_A>;
#[doc = "eUSCI_B clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSSEL_A {
    #[doc = "0: Reserved"]
    UCSSEL_0 = 0,
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
            0 => UCSSEL_A::UCSSEL_0,
            1 => UCSSEL_A::ACLK,
            2 => UCSSEL_A::SMCLK,
            3 => UCSSEL_A::UCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCSSEL_0`"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_0
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
    crate::FieldWriterSafe<'a, u16, UCB0CTLW0_SPI_SPEC, u8, UCSSEL_A, 2, O>;
impl<'a, const O: u8> UCSSEL_W<'a, O> {
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_0)
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
pub type UCSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UCSYNC_A, O>;
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
#[doc = "Field `UCMODE` reader - eUSCI mode"]
pub type UCMODE_R = crate::FieldReader<u8, UCMODE_A>;
#[doc = "eUSCI mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: 3-pin SPI"]
    UCMODE_0 = 0,
    #[doc = "1: 4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1"]
    UCMODE_1 = 1,
    #[doc = "2: 4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0"]
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
#[doc = "Field `UCMODE` writer - eUSCI mode"]
pub type UCMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCB0CTLW0_SPI_SPEC, u8, UCMODE_A, 2, O>;
impl<'a, const O: u8> UCMODE_W<'a, O> {
    #[doc = "3-pin SPI"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0"]
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
pub type UCMST_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UCMST_A, O>;
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
#[doc = "Field `UC7BIT` reader - Character length"]
pub type UC7BIT_R = crate::BitReader<UC7BIT_A>;
#[doc = "Character length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UC7BIT_A {
    #[doc = "0: 8-bit data"]
    _8BIT = 0,
    #[doc = "1: 7-bit data"]
    _7BIT = 1,
}
impl From<UC7BIT_A> for bool {
    #[inline(always)]
    fn from(variant: UC7BIT_A) -> Self {
        variant as u8 != 0
    }
}
impl UC7BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UC7BIT_A {
        match self.bits {
            false => UC7BIT_A::_8BIT,
            true => UC7BIT_A::_7BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == UC7BIT_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_7BIT`"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == UC7BIT_A::_7BIT
    }
}
#[doc = "Field `UC7BIT` writer - Character length"]
pub type UC7BIT_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UC7BIT_A, O>;
impl<'a, const O: u8> UC7BIT_W<'a, O> {
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(UC7BIT_A::_8BIT)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut W {
        self.variant(UC7BIT_A::_7BIT)
    }
}
#[doc = "Field `UCMSB` reader - MSB first select"]
pub type UCMSB_R = crate::BitReader<UCMSB_A>;
#[doc = "MSB first select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCMSB_A {
    #[doc = "0: LSB first"]
    UCMSB_0 = 0,
    #[doc = "1: MSB first"]
    UCMSB_1 = 1,
}
impl From<UCMSB_A> for bool {
    #[inline(always)]
    fn from(variant: UCMSB_A) -> Self {
        variant as u8 != 0
    }
}
impl UCMSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMSB_A {
        match self.bits {
            false => UCMSB_A::UCMSB_0,
            true => UCMSB_A::UCMSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCMSB_0`"]
    #[inline(always)]
    pub fn is_ucmsb_0(&self) -> bool {
        *self == UCMSB_A::UCMSB_0
    }
    #[doc = "Checks if the value of the field is `UCMSB_1`"]
    #[inline(always)]
    pub fn is_ucmsb_1(&self) -> bool {
        *self == UCMSB_A::UCMSB_1
    }
}
#[doc = "Field `UCMSB` writer - MSB first select"]
pub type UCMSB_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UCMSB_A, O>;
impl<'a, const O: u8> UCMSB_W<'a, O> {
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn ucmsb_0(self) -> &'a mut W {
        self.variant(UCMSB_A::UCMSB_0)
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn ucmsb_1(self) -> &'a mut W {
        self.variant(UCMSB_A::UCMSB_1)
    }
}
#[doc = "Field `UCCKPL` reader - Clock polarity select"]
pub type UCCKPL_R = crate::BitReader<UCCKPL_A>;
#[doc = "Clock polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCCKPL_A {
    #[doc = "0: The inactive state is low"]
    LOW = 0,
    #[doc = "1: The inactive state is high"]
    HIGH = 1,
}
impl From<UCCKPL_A> for bool {
    #[inline(always)]
    fn from(variant: UCCKPL_A) -> Self {
        variant as u8 != 0
    }
}
impl UCCKPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCCKPL_A {
        match self.bits {
            false => UCCKPL_A::LOW,
            true => UCCKPL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == UCCKPL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == UCCKPL_A::HIGH
    }
}
#[doc = "Field `UCCKPL` writer - Clock polarity select"]
pub type UCCKPL_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UCCKPL_A, O>;
impl<'a, const O: u8> UCCKPL_W<'a, O> {
    #[doc = "The inactive state is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(UCCKPL_A::LOW)
    }
    #[doc = "The inactive state is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(UCCKPL_A::HIGH)
    }
}
#[doc = "Field `UCCKPH` reader - Clock phase select"]
pub type UCCKPH_R = crate::BitReader<UCCKPH_A>;
#[doc = "Clock phase select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCCKPH_A {
    #[doc = "0: Data is changed on the first UCLK edge and captured on the following edge."]
    UCCKPH_0 = 0,
    #[doc = "1: Data is captured on the first UCLK edge and changed on the following edge."]
    UCCKPH_1 = 1,
}
impl From<UCCKPH_A> for bool {
    #[inline(always)]
    fn from(variant: UCCKPH_A) -> Self {
        variant as u8 != 0
    }
}
impl UCCKPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCCKPH_A {
        match self.bits {
            false => UCCKPH_A::UCCKPH_0,
            true => UCCKPH_A::UCCKPH_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCCKPH_0`"]
    #[inline(always)]
    pub fn is_ucckph_0(&self) -> bool {
        *self == UCCKPH_A::UCCKPH_0
    }
    #[doc = "Checks if the value of the field is `UCCKPH_1`"]
    #[inline(always)]
    pub fn is_ucckph_1(&self) -> bool {
        *self == UCCKPH_A::UCCKPH_1
    }
}
#[doc = "Field `UCCKPH` writer - Clock phase select"]
pub type UCCKPH_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0CTLW0_SPI_SPEC, UCCKPH_A, O>;
impl<'a, const O: u8> UCCKPH_W<'a, O> {
    #[doc = "Data is changed on the first UCLK edge and captured on the following edge."]
    #[inline(always)]
    pub fn ucckph_0(self) -> &'a mut W {
        self.variant(UCCKPH_A::UCCKPH_0)
    }
    #[doc = "Data is captured on the first UCLK edge and changed on the following edge."]
    #[inline(always)]
    pub fn ucckph_1(self) -> &'a mut W {
        self.variant(UCCKPH_A::UCCKPH_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&self) -> UCSTEM_R {
        UCSTEM_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bits 9:10 - eUSCI mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&self) -> UCCKPL_R {
        UCCKPL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&self) -> UCCKPH_R {
        UCCKPH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W<0> {
        UCSWRST_W::new(self)
    }
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&mut self) -> UCSTEM_W<1> {
        UCSTEM_W::new(self)
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
    #[doc = "Bits 9:10 - eUSCI mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W<9> {
        UCMODE_W::new(self)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W<11> {
        UCMST_W::new(self)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> UC7BIT_W<12> {
        UC7BIT_W::new(self)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UCMSB_W<13> {
        UCMSB_W::new(self)
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&mut self) -> UCCKPL_W<14> {
        UCCKPL_W::new(self)
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&mut self) -> UCCKPH_W<15> {
        UCCKPH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctlw0_spi](index.html) module"]
pub struct UCB0CTLW0_SPI_SPEC;
impl crate::RegisterSpec for UCB0CTLW0_SPI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0ctlw0_spi::R](R) reader structure"]
impl crate::Readable for UCB0CTLW0_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ctlw0_spi::W](W) writer structure"]
impl crate::Writable for UCB0CTLW0_SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0CTLW0_SPI to value 0"]
impl crate::Resettable for UCB0CTLW0_SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
