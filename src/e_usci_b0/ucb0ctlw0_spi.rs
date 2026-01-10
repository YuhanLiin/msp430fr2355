#[doc = "Register `UCB0CTLW0_SPI` reader"]
pub type R = crate::R<Ucb0ctlw0SpiSpec>;
#[doc = "Register `UCB0CTLW0_SPI` writer"]
pub type W = crate::W<Ucb0ctlw0SpiSpec>;
#[doc = "Software reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucswrst {
    #[doc = "0: Disabled. eUSCI_B reset released for operation"]
    Disable = 0,
    #[doc = "1: Enabled. eUSCI_B logic held in reset state"]
    Enable = 1,
}
impl From<Ucswrst> for bool {
    #[inline(always)]
    fn from(variant: Ucswrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UcswrstR = crate::BitReader<Ucswrst>;
impl UcswrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucswrst {
        match self.bits {
            false => Ucswrst::Disable,
            true => Ucswrst::Enable,
        }
    }
    #[doc = "Disabled. eUSCI_B reset released for operation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ucswrst::Disable
    }
    #[doc = "Enabled. eUSCI_B logic held in reset state"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ucswrst::Enable
    }
}
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG, Ucswrst>;
impl<'a, REG> UcswrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. eUSCI_B reset released for operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswrst::Disable)
    }
    #[doc = "Enabled. eUSCI_B logic held in reset state"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswrst::Enable)
    }
}
#[doc = "STE mode select in master mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucstem {
    #[doc = "0: STE pin is used to prevent conflicts with other masters"]
    Ucstem0 = 0,
    #[doc = "1: STE pin is used to generate the enable signal for a 4-wire slave"]
    Ucstem1 = 1,
}
impl From<Ucstem> for bool {
    #[inline(always)]
    fn from(variant: Ucstem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTEM` reader - STE mode select in master mode."]
pub type UcstemR = crate::BitReader<Ucstem>;
impl UcstemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucstem {
        match self.bits {
            false => Ucstem::Ucstem0,
            true => Ucstem::Ucstem1,
        }
    }
    #[doc = "STE pin is used to prevent conflicts with other masters"]
    #[inline(always)]
    pub fn is_ucstem_0(&self) -> bool {
        *self == Ucstem::Ucstem0
    }
    #[doc = "STE pin is used to generate the enable signal for a 4-wire slave"]
    #[inline(always)]
    pub fn is_ucstem_1(&self) -> bool {
        *self == Ucstem::Ucstem1
    }
}
#[doc = "Field `UCSTEM` writer - STE mode select in master mode."]
pub type UcstemW<'a, REG> = crate::BitWriter<'a, REG, Ucstem>;
impl<'a, REG> UcstemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STE pin is used to prevent conflicts with other masters"]
    #[inline(always)]
    pub fn ucstem_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstem::Ucstem0)
    }
    #[doc = "STE pin is used to generate the enable signal for a 4-wire slave"]
    #[inline(always)]
    pub fn ucstem_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstem::Ucstem1)
    }
}
#[doc = "eUSCI_B clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucssel {
    #[doc = "0: Reserved"]
    Ucssel0 = 0,
    #[doc = "1: ACLK"]
    Aclk = 1,
    #[doc = "2: SMCLK"]
    Smclk = 2,
    #[doc = "3: SMCLK"]
    Ucssel3 = 3,
}
impl From<Ucssel> for u8 {
    #[inline(always)]
    fn from(variant: Ucssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucssel {
    type Ux = u8;
}
impl crate::IsEnum for Ucssel {}
#[doc = "Field `UCSSEL` reader - eUSCI_B clock source select"]
pub type UcsselR = crate::FieldReader<Ucssel>;
impl UcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucssel {
        match self.bits {
            0 => Ucssel::Ucssel0,
            1 => Ucssel::Aclk,
            2 => Ucssel::Smclk,
            3 => Ucssel::Ucssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == Ucssel::Ucssel0
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Ucssel::Aclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Ucssel::Smclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == Ucssel::Ucssel3
    }
}
#[doc = "Field `UCSSEL` writer - eUSCI_B clock source select"]
pub type UcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucssel, crate::Safe>;
impl<'a, REG> UcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Aclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Smclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel3)
    }
}
#[doc = "Synchronous mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsync {
    #[doc = "0: Asynchronous mode"]
    Async = 0,
    #[doc = "1: Synchronous mode"]
    Sync = 1,
}
impl From<Ucsync> for bool {
    #[inline(always)]
    fn from(variant: Ucsync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSYNC` reader - Synchronous mode enable"]
pub type UcsyncR = crate::BitReader<Ucsync>;
impl UcsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsync {
        match self.bits {
            false => Ucsync::Async,
            true => Ucsync::Sync,
        }
    }
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Ucsync::Async
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Ucsync::Sync
    }
}
#[doc = "Field `UCSYNC` writer - Synchronous mode enable"]
pub type UcsyncW<'a, REG> = crate::BitWriter<'a, REG, Ucsync>;
impl<'a, REG> UcsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsync::Async)
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsync::Sync)
    }
}
#[doc = "eUSCI mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucmode {
    #[doc = "0: 3-pin SPI"]
    Ucmode0 = 0,
    #[doc = "1: 4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1"]
    Ucmode1 = 1,
    #[doc = "2: 4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0"]
    Ucmode2 = 2,
    #[doc = "3: I2C mode"]
    Ucmode3 = 3,
}
impl From<Ucmode> for u8 {
    #[inline(always)]
    fn from(variant: Ucmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucmode {
    type Ux = u8;
}
impl crate::IsEnum for Ucmode {}
#[doc = "Field `UCMODE` reader - eUSCI mode"]
pub type UcmodeR = crate::FieldReader<Ucmode>;
impl UcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmode {
        match self.bits {
            0 => Ucmode::Ucmode0,
            1 => Ucmode::Ucmode1,
            2 => Ucmode::Ucmode2,
            3 => Ucmode::Ucmode3,
            _ => unreachable!(),
        }
    }
    #[doc = "3-pin SPI"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == Ucmode::Ucmode0
    }
    #[doc = "4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == Ucmode::Ucmode1
    }
    #[doc = "4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == Ucmode::Ucmode2
    }
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == Ucmode::Ucmode3
    }
}
#[doc = "Field `UCMODE` writer - eUSCI mode"]
pub type UcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucmode, crate::Safe>;
impl<'a, REG> UcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3-pin SPI"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode0)
    }
    #[doc = "4-pin SPI with UCxSTE active high: Slave enabled when UCxSTE = 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode1)
    }
    #[doc = "4-pin SPI with UCxSTE active low: Slave enabled when UCxSTE = 0"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode2)
    }
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode3)
    }
}
#[doc = "Master mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucmst {
    #[doc = "0: Slave mode"]
    Slave = 0,
    #[doc = "1: Master mode"]
    Master = 1,
}
impl From<Ucmst> for bool {
    #[inline(always)]
    fn from(variant: Ucmst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCMST` reader - Master mode select"]
pub type UcmstR = crate::BitReader<Ucmst>;
impl UcmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmst {
        match self.bits {
            false => Ucmst::Slave,
            true => Ucmst::Master,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Ucmst::Slave
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Ucmst::Master
    }
}
#[doc = "Field `UCMST` writer - Master mode select"]
pub type UcmstW<'a, REG> = crate::BitWriter<'a, REG, Ucmst>;
impl<'a, REG> UcmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmst::Slave)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmst::Master)
    }
}
#[doc = "Character length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uc7bit {
    #[doc = "0: 8-bit data"]
    _8bit = 0,
    #[doc = "1: 7-bit data"]
    _7bit = 1,
}
impl From<Uc7bit> for bool {
    #[inline(always)]
    fn from(variant: Uc7bit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UC7BIT` reader - Character length"]
pub type Uc7bitR = crate::BitReader<Uc7bit>;
impl Uc7bitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uc7bit {
        match self.bits {
            false => Uc7bit::_8bit,
            true => Uc7bit::_7bit,
        }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Uc7bit::_8bit
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Uc7bit::_7bit
    }
}
#[doc = "Field `UC7BIT` writer - Character length"]
pub type Uc7bitW<'a, REG> = crate::BitWriter<'a, REG, Uc7bit>;
impl<'a, REG> Uc7bitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Uc7bit::_8bit)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Uc7bit::_7bit)
    }
}
#[doc = "MSB first select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucmsb {
    #[doc = "0: LSB first"]
    Ucmsb0 = 0,
    #[doc = "1: MSB first"]
    Ucmsb1 = 1,
}
impl From<Ucmsb> for bool {
    #[inline(always)]
    fn from(variant: Ucmsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCMSB` reader - MSB first select"]
pub type UcmsbR = crate::BitReader<Ucmsb>;
impl UcmsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmsb {
        match self.bits {
            false => Ucmsb::Ucmsb0,
            true => Ucmsb::Ucmsb1,
        }
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_ucmsb_0(&self) -> bool {
        *self == Ucmsb::Ucmsb0
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_ucmsb_1(&self) -> bool {
        *self == Ucmsb::Ucmsb1
    }
}
#[doc = "Field `UCMSB` writer - MSB first select"]
pub type UcmsbW<'a, REG> = crate::BitWriter<'a, REG, Ucmsb>;
impl<'a, REG> UcmsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn ucmsb_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmsb::Ucmsb0)
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn ucmsb_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmsb::Ucmsb1)
    }
}
#[doc = "Clock polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucckpl {
    #[doc = "0: The inactive state is low"]
    Low = 0,
    #[doc = "1: The inactive state is high"]
    High = 1,
}
impl From<Ucckpl> for bool {
    #[inline(always)]
    fn from(variant: Ucckpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCCKPL` reader - Clock polarity select"]
pub type UcckplR = crate::BitReader<Ucckpl>;
impl UcckplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucckpl {
        match self.bits {
            false => Ucckpl::Low,
            true => Ucckpl::High,
        }
    }
    #[doc = "The inactive state is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ucckpl::Low
    }
    #[doc = "The inactive state is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ucckpl::High
    }
}
#[doc = "Field `UCCKPL` writer - Clock polarity select"]
pub type UcckplW<'a, REG> = crate::BitWriter<'a, REG, Ucckpl>;
impl<'a, REG> UcckplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The inactive state is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ucckpl::Low)
    }
    #[doc = "The inactive state is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ucckpl::High)
    }
}
#[doc = "Clock phase select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucckph {
    #[doc = "0: Data is changed on the first UCLK edge and captured on the following edge."]
    Ucckph0 = 0,
    #[doc = "1: Data is captured on the first UCLK edge and changed on the following edge."]
    Ucckph1 = 1,
}
impl From<Ucckph> for bool {
    #[inline(always)]
    fn from(variant: Ucckph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCCKPH` reader - Clock phase select"]
pub type UcckphR = crate::BitReader<Ucckph>;
impl UcckphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucckph {
        match self.bits {
            false => Ucckph::Ucckph0,
            true => Ucckph::Ucckph1,
        }
    }
    #[doc = "Data is changed on the first UCLK edge and captured on the following edge."]
    #[inline(always)]
    pub fn is_ucckph_0(&self) -> bool {
        *self == Ucckph::Ucckph0
    }
    #[doc = "Data is captured on the first UCLK edge and changed on the following edge."]
    #[inline(always)]
    pub fn is_ucckph_1(&self) -> bool {
        *self == Ucckph::Ucckph1
    }
}
#[doc = "Field `UCCKPH` writer - Clock phase select"]
pub type UcckphW<'a, REG> = crate::BitWriter<'a, REG, Ucckph>;
impl<'a, REG> UcckphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is changed on the first UCLK edge and captured on the following edge."]
    #[inline(always)]
    pub fn ucckph_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucckph::Ucckph0)
    }
    #[doc = "Data is captured on the first UCLK edge and changed on the following edge."]
    #[inline(always)]
    pub fn ucckph_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucckph::Ucckph1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&self) -> UcstemR {
        UcstemR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UcsselR {
        UcsselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UcsyncR {
        UcsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UcmodeR {
        UcmodeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UcmstR {
        UcmstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&self) -> Uc7bitR {
        Uc7bitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UcmsbR {
        UcmsbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&self) -> UcckplR {
        UcckplR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&self) -> UcckphR {
        UcckphR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<'_, Ucb0ctlw0SpiSpec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&mut self) -> UcstemW<'_, Ucb0ctlw0SpiSpec> {
        UcstemW::new(self, 1)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<'_, Ucb0ctlw0SpiSpec> {
        UcsselW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Ucb0ctlw0SpiSpec> {
        UcsyncW::new(self, 8)
    }
    #[doc = "Bits 9:10 - eUSCI mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Ucb0ctlw0SpiSpec> {
        UcmodeW::new(self, 9)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UcmstW<'_, Ucb0ctlw0SpiSpec> {
        UcmstW::new(self, 11)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> Uc7bitW<'_, Ucb0ctlw0SpiSpec> {
        Uc7bitW::new(self, 12)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UcmsbW<'_, Ucb0ctlw0SpiSpec> {
        UcmsbW::new(self, 13)
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&mut self) -> UcckplW<'_, Ucb0ctlw0SpiSpec> {
        UcckplW::new(self, 14)
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&mut self) -> UcckphW<'_, Ucb0ctlw0SpiSpec> {
        UcckphW::new(self, 15)
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw0_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw0_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ctlw0SpiSpec;
impl crate::RegisterSpec for Ucb0ctlw0SpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ctlw0_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0ctlw0SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctlw0_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0ctlw0SpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0CTLW0_SPI to value 0"]
impl crate::Resettable for Ucb0ctlw0SpiSpec {}
