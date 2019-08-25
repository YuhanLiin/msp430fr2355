#[doc = "Reader of register UCB0CTLW0_SPI"]
pub type R = crate::R<u16, super::UCB0CTLW0_SPI>;
#[doc = "Writer for register UCB0CTLW0_SPI"]
pub type W = crate::W<u16, super::UCB0CTLW0_SPI>;
#[doc = "Register UCB0CTLW0_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0CTLW0_SPI {
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
#[doc = "STE mode select in master mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTEM_A {
    #[doc = "0: STE pin is used to prevent conflicts with other masters"]
    UCSTEM_0,
    #[doc = "1: STE pin is used to generate the enable signal for a 4-wire slave"]
    UCSTEM_1,
}
impl From<UCSTEM_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTEM_A) -> Self {
        match variant {
            UCSTEM_A::UCSTEM_0 => false,
            UCSTEM_A::UCSTEM_1 => true,
        }
    }
}
#[doc = "Reader of field `UCSTEM`"]
pub type UCSTEM_R = crate::R<bool, UCSTEM_A>;
impl UCSTEM_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCSTEM`"]
pub struct UCSTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "eUSCI_B clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSSEL_A {
    #[doc = "0: Reserved"]
    UCSSEL_0,
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
            UCSSEL_A::UCSSEL_0 => 0,
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
#[doc = "eUSCI mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMODE_A {
    #[doc = "0: 3-pin SPI"]
    UCMODE_0,
    #[doc = "1: 4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1"]
    UCMODE_1,
    #[doc = "2: 4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0"]
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
#[doc = "Character length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UC7BIT_A {
    #[doc = "0: 8-bit data"]
    _8BIT,
    #[doc = "1: 7-bit data"]
    _7BIT,
}
impl From<UC7BIT_A> for bool {
    #[inline(always)]
    fn from(variant: UC7BIT_A) -> Self {
        match variant {
            UC7BIT_A::_8BIT => false,
            UC7BIT_A::_7BIT => true,
        }
    }
}
#[doc = "Reader of field `UC7BIT`"]
pub type UC7BIT_R = crate::R<bool, UC7BIT_A>;
impl UC7BIT_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UC7BIT`"]
pub struct UC7BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UC7BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UC7BIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "MSB first select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMSB_A {
    #[doc = "0: LSB first"]
    UCMSB_0,
    #[doc = "1: MSB first"]
    UCMSB_1,
}
impl From<UCMSB_A> for bool {
    #[inline(always)]
    fn from(variant: UCMSB_A) -> Self {
        match variant {
            UCMSB_A::UCMSB_0 => false,
            UCMSB_A::UCMSB_1 => true,
        }
    }
}
#[doc = "Reader of field `UCMSB`"]
pub type UCMSB_R = crate::R<bool, UCMSB_A>;
impl UCMSB_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCMSB`"]
pub struct UCMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Clock polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCCKPL_A {
    #[doc = "0: The inactive state is low"]
    LOW,
    #[doc = "1: The inactive state is high"]
    HIGH,
}
impl From<UCCKPL_A> for bool {
    #[inline(always)]
    fn from(variant: UCCKPL_A) -> Self {
        match variant {
            UCCKPL_A::LOW => false,
            UCCKPL_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `UCCKPL`"]
pub type UCCKPL_R = crate::R<bool, UCCKPL_A>;
impl UCCKPL_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCCKPL`"]
pub struct UCCKPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCKPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCCKPL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Clock phase select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCCKPH_A {
    #[doc = "0: Data is changed on the first UCLK edge and captured on the following edge."]
    UCCKPH_0,
    #[doc = "1: Data is captured on the first UCLK edge and changed on the following edge."]
    UCCKPH_1,
}
impl From<UCCKPH_A> for bool {
    #[inline(always)]
    fn from(variant: UCCKPH_A) -> Self {
        match variant {
            UCCKPH_A::UCCKPH_0 => false,
            UCCKPH_A::UCCKPH_1 => true,
        }
    }
}
#[doc = "Reader of field `UCCKPH`"]
pub type UCCKPH_R = crate::R<bool, UCCKPH_A>;
impl UCCKPH_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCCKPH`"]
pub struct UCCKPH_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCKPH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCCKPH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&self) -> UCSTEM_R {
        UCSTEM_R::new(((self.bits >> 1) & 0x01) != 0)
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
    #[doc = "Bits 9:10 - eUSCI mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&self) -> UCCKPL_R {
        UCCKPL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&self) -> UCCKPH_R {
        UCCKPH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W {
        UCSWRST_W { w: self }
    }
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&mut self) -> UCSTEM_W {
        UCSTEM_W { w: self }
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
    #[doc = "Bits 9:10 - eUSCI mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W {
        UCMODE_W { w: self }
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W {
        UCMST_W { w: self }
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> UC7BIT_W {
        UC7BIT_W { w: self }
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UCMSB_W {
        UCMSB_W { w: self }
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&mut self) -> UCCKPL_W {
        UCCKPL_W { w: self }
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&mut self) -> UCCKPH_W {
        UCCKPH_W { w: self }
    }
}
