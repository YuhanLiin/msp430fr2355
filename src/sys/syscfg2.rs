#[doc = "Register `SYSCFG2` reader"]
pub type R = crate::R<Syscfg2Spec>;
#[doc = "Register `SYSCFG2` writer"]
pub type W = crate::W<Syscfg2Spec>;
#[doc = "ADC input A0 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl0 {
    #[doc = "0: ADC input A0 disabled"]
    Adcpctl0_0 = 0,
    #[doc = "1: ADC input A0 enabled"]
    Adcpctl0_1 = 1,
}
impl From<Adcpctl0> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL0` reader - ADC input A0 pin select"]
pub type Adcpctl0R = crate::BitReader<Adcpctl0>;
impl Adcpctl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl0 {
        match self.bits {
            false => Adcpctl0::Adcpctl0_0,
            true => Adcpctl0::Adcpctl0_1,
        }
    }
    #[doc = "ADC input A0 disabled"]
    #[inline(always)]
    pub fn is_adcpctl0_0(&self) -> bool {
        *self == Adcpctl0::Adcpctl0_0
    }
    #[doc = "ADC input A0 enabled"]
    #[inline(always)]
    pub fn is_adcpctl0_1(&self) -> bool {
        *self == Adcpctl0::Adcpctl0_1
    }
}
#[doc = "Field `ADCPCTL0` writer - ADC input A0 pin select"]
pub type Adcpctl0W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl0>;
impl<'a, REG> Adcpctl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A0 disabled"]
    #[inline(always)]
    pub fn adcpctl0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl0::Adcpctl0_0)
    }
    #[doc = "ADC input A0 enabled"]
    #[inline(always)]
    pub fn adcpctl0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl0::Adcpctl0_1)
    }
}
#[doc = "ADC input A1 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl1 {
    #[doc = "0: ADC input A1 disabled"]
    Adcpctl1_0 = 0,
    #[doc = "1: ADC input A1 enabled"]
    Adcpctl1_1 = 1,
}
impl From<Adcpctl1> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL1` reader - ADC input A1 pin select"]
pub type Adcpctl1R = crate::BitReader<Adcpctl1>;
impl Adcpctl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl1 {
        match self.bits {
            false => Adcpctl1::Adcpctl1_0,
            true => Adcpctl1::Adcpctl1_1,
        }
    }
    #[doc = "ADC input A1 disabled"]
    #[inline(always)]
    pub fn is_adcpctl1_0(&self) -> bool {
        *self == Adcpctl1::Adcpctl1_0
    }
    #[doc = "ADC input A1 enabled"]
    #[inline(always)]
    pub fn is_adcpctl1_1(&self) -> bool {
        *self == Adcpctl1::Adcpctl1_1
    }
}
#[doc = "Field `ADCPCTL1` writer - ADC input A1 pin select"]
pub type Adcpctl1W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl1>;
impl<'a, REG> Adcpctl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A1 disabled"]
    #[inline(always)]
    pub fn adcpctl1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl1::Adcpctl1_0)
    }
    #[doc = "ADC input A1 enabled"]
    #[inline(always)]
    pub fn adcpctl1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl1::Adcpctl1_1)
    }
}
#[doc = "ADC input A2 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl2 {
    #[doc = "0: ADC input A2 disabled"]
    Adcpctl2_0 = 0,
    #[doc = "1: ADC input A2 enabled"]
    Adcpctl2_1 = 1,
}
impl From<Adcpctl2> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL2` reader - ADC input A2 pin select"]
pub type Adcpctl2R = crate::BitReader<Adcpctl2>;
impl Adcpctl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl2 {
        match self.bits {
            false => Adcpctl2::Adcpctl2_0,
            true => Adcpctl2::Adcpctl2_1,
        }
    }
    #[doc = "ADC input A2 disabled"]
    #[inline(always)]
    pub fn is_adcpctl2_0(&self) -> bool {
        *self == Adcpctl2::Adcpctl2_0
    }
    #[doc = "ADC input A2 enabled"]
    #[inline(always)]
    pub fn is_adcpctl2_1(&self) -> bool {
        *self == Adcpctl2::Adcpctl2_1
    }
}
#[doc = "Field `ADCPCTL2` writer - ADC input A2 pin select"]
pub type Adcpctl2W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl2>;
impl<'a, REG> Adcpctl2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A2 disabled"]
    #[inline(always)]
    pub fn adcpctl2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl2::Adcpctl2_0)
    }
    #[doc = "ADC input A2 enabled"]
    #[inline(always)]
    pub fn adcpctl2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl2::Adcpctl2_1)
    }
}
#[doc = "ADC input A3 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl3 {
    #[doc = "0: ADC input A3 disabled"]
    Adcpctl3_0 = 0,
    #[doc = "1: ADC input A3 enabled"]
    Adcpctl3_1 = 1,
}
impl From<Adcpctl3> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL3` reader - ADC input A3 pin select"]
pub type Adcpctl3R = crate::BitReader<Adcpctl3>;
impl Adcpctl3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl3 {
        match self.bits {
            false => Adcpctl3::Adcpctl3_0,
            true => Adcpctl3::Adcpctl3_1,
        }
    }
    #[doc = "ADC input A3 disabled"]
    #[inline(always)]
    pub fn is_adcpctl3_0(&self) -> bool {
        *self == Adcpctl3::Adcpctl3_0
    }
    #[doc = "ADC input A3 enabled"]
    #[inline(always)]
    pub fn is_adcpctl3_1(&self) -> bool {
        *self == Adcpctl3::Adcpctl3_1
    }
}
#[doc = "Field `ADCPCTL3` writer - ADC input A3 pin select"]
pub type Adcpctl3W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl3>;
impl<'a, REG> Adcpctl3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A3 disabled"]
    #[inline(always)]
    pub fn adcpctl3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl3::Adcpctl3_0)
    }
    #[doc = "ADC input A3 enabled"]
    #[inline(always)]
    pub fn adcpctl3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl3::Adcpctl3_1)
    }
}
#[doc = "ADC input A4 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl4 {
    #[doc = "0: ADC input A4 disabled"]
    Adcpctl4_0 = 0,
    #[doc = "1: ADC input A4 enabled"]
    Adcpctl4_1 = 1,
}
impl From<Adcpctl4> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL4` reader - ADC input A4 pin select"]
pub type Adcpctl4R = crate::BitReader<Adcpctl4>;
impl Adcpctl4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl4 {
        match self.bits {
            false => Adcpctl4::Adcpctl4_0,
            true => Adcpctl4::Adcpctl4_1,
        }
    }
    #[doc = "ADC input A4 disabled"]
    #[inline(always)]
    pub fn is_adcpctl4_0(&self) -> bool {
        *self == Adcpctl4::Adcpctl4_0
    }
    #[doc = "ADC input A4 enabled"]
    #[inline(always)]
    pub fn is_adcpctl4_1(&self) -> bool {
        *self == Adcpctl4::Adcpctl4_1
    }
}
#[doc = "Field `ADCPCTL4` writer - ADC input A4 pin select"]
pub type Adcpctl4W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl4>;
impl<'a, REG> Adcpctl4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A4 disabled"]
    #[inline(always)]
    pub fn adcpctl4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl4::Adcpctl4_0)
    }
    #[doc = "ADC input A4 enabled"]
    #[inline(always)]
    pub fn adcpctl4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl4::Adcpctl4_1)
    }
}
#[doc = "ADC input A5 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl5 {
    #[doc = "0: ADC input A5 disabled"]
    Adcpctl5_0 = 0,
    #[doc = "1: ADC input A5 enabled"]
    Adcpctl5_1 = 1,
}
impl From<Adcpctl5> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL5` reader - ADC input A5 pin select"]
pub type Adcpctl5R = crate::BitReader<Adcpctl5>;
impl Adcpctl5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl5 {
        match self.bits {
            false => Adcpctl5::Adcpctl5_0,
            true => Adcpctl5::Adcpctl5_1,
        }
    }
    #[doc = "ADC input A5 disabled"]
    #[inline(always)]
    pub fn is_adcpctl5_0(&self) -> bool {
        *self == Adcpctl5::Adcpctl5_0
    }
    #[doc = "ADC input A5 enabled"]
    #[inline(always)]
    pub fn is_adcpctl5_1(&self) -> bool {
        *self == Adcpctl5::Adcpctl5_1
    }
}
#[doc = "Field `ADCPCTL5` writer - ADC input A5 pin select"]
pub type Adcpctl5W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl5>;
impl<'a, REG> Adcpctl5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A5 disabled"]
    #[inline(always)]
    pub fn adcpctl5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl5::Adcpctl5_0)
    }
    #[doc = "ADC input A5 enabled"]
    #[inline(always)]
    pub fn adcpctl5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl5::Adcpctl5_1)
    }
}
#[doc = "ADC input A6 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl6 {
    #[doc = "0: ADC input A6 disabled"]
    Adcpctl6_0 = 0,
    #[doc = "1: ADC input A6 enabled"]
    Adcpctl6_1 = 1,
}
impl From<Adcpctl6> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL6` reader - ADC input A6 pin select"]
pub type Adcpctl6R = crate::BitReader<Adcpctl6>;
impl Adcpctl6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl6 {
        match self.bits {
            false => Adcpctl6::Adcpctl6_0,
            true => Adcpctl6::Adcpctl6_1,
        }
    }
    #[doc = "ADC input A6 disabled"]
    #[inline(always)]
    pub fn is_adcpctl6_0(&self) -> bool {
        *self == Adcpctl6::Adcpctl6_0
    }
    #[doc = "ADC input A6 enabled"]
    #[inline(always)]
    pub fn is_adcpctl6_1(&self) -> bool {
        *self == Adcpctl6::Adcpctl6_1
    }
}
#[doc = "Field `ADCPCTL6` writer - ADC input A6 pin select"]
pub type Adcpctl6W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl6>;
impl<'a, REG> Adcpctl6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A6 disabled"]
    #[inline(always)]
    pub fn adcpctl6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl6::Adcpctl6_0)
    }
    #[doc = "ADC input A6 enabled"]
    #[inline(always)]
    pub fn adcpctl6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl6::Adcpctl6_1)
    }
}
#[doc = "ADC input A7 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl7 {
    #[doc = "0: ADC input A7 disabled"]
    Adcpctl7_0 = 0,
    #[doc = "1: ADC input A7 enabled"]
    Adcpctl7_1 = 1,
}
impl From<Adcpctl7> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL7` reader - ADC input A7 pin select"]
pub type Adcpctl7R = crate::BitReader<Adcpctl7>;
impl Adcpctl7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl7 {
        match self.bits {
            false => Adcpctl7::Adcpctl7_0,
            true => Adcpctl7::Adcpctl7_1,
        }
    }
    #[doc = "ADC input A7 disabled"]
    #[inline(always)]
    pub fn is_adcpctl7_0(&self) -> bool {
        *self == Adcpctl7::Adcpctl7_0
    }
    #[doc = "ADC input A7 enabled"]
    #[inline(always)]
    pub fn is_adcpctl7_1(&self) -> bool {
        *self == Adcpctl7::Adcpctl7_1
    }
}
#[doc = "Field `ADCPCTL7` writer - ADC input A7 pin select"]
pub type Adcpctl7W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl7>;
impl<'a, REG> Adcpctl7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A7 disabled"]
    #[inline(always)]
    pub fn adcpctl7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl7::Adcpctl7_0)
    }
    #[doc = "ADC input A7 enabled"]
    #[inline(always)]
    pub fn adcpctl7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl7::Adcpctl7_1)
    }
}
#[doc = "ADC input A8 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl8 {
    #[doc = "0: ADC input A8 disabled"]
    Adcpctl8_0 = 0,
    #[doc = "1: ADC input A8 enabled"]
    Adcpctl8_1 = 1,
}
impl From<Adcpctl8> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL8` reader - ADC input A8 pin select"]
pub type Adcpctl8R = crate::BitReader<Adcpctl8>;
impl Adcpctl8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl8 {
        match self.bits {
            false => Adcpctl8::Adcpctl8_0,
            true => Adcpctl8::Adcpctl8_1,
        }
    }
    #[doc = "ADC input A8 disabled"]
    #[inline(always)]
    pub fn is_adcpctl8_0(&self) -> bool {
        *self == Adcpctl8::Adcpctl8_0
    }
    #[doc = "ADC input A8 enabled"]
    #[inline(always)]
    pub fn is_adcpctl8_1(&self) -> bool {
        *self == Adcpctl8::Adcpctl8_1
    }
}
#[doc = "Field `ADCPCTL8` writer - ADC input A8 pin select"]
pub type Adcpctl8W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl8>;
impl<'a, REG> Adcpctl8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A8 disabled"]
    #[inline(always)]
    pub fn adcpctl8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl8::Adcpctl8_0)
    }
    #[doc = "ADC input A8 enabled"]
    #[inline(always)]
    pub fn adcpctl8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl8::Adcpctl8_1)
    }
}
#[doc = "ADC input A9 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcpctl9 {
    #[doc = "0: ADC input A9 disabled"]
    Adcpctl9_0 = 0,
    #[doc = "1: ADC input A9 enabled"]
    Adcpctl9_1 = 1,
}
impl From<Adcpctl9> for bool {
    #[inline(always)]
    fn from(variant: Adcpctl9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCPCTL9` reader - ADC input A9 pin select"]
pub type Adcpctl9R = crate::BitReader<Adcpctl9>;
impl Adcpctl9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcpctl9 {
        match self.bits {
            false => Adcpctl9::Adcpctl9_0,
            true => Adcpctl9::Adcpctl9_1,
        }
    }
    #[doc = "ADC input A9 disabled"]
    #[inline(always)]
    pub fn is_adcpctl9_0(&self) -> bool {
        *self == Adcpctl9::Adcpctl9_0
    }
    #[doc = "ADC input A9 enabled"]
    #[inline(always)]
    pub fn is_adcpctl9_1(&self) -> bool {
        *self == Adcpctl9::Adcpctl9_1
    }
}
#[doc = "Field `ADCPCTL9` writer - ADC input A9 pin select"]
pub type Adcpctl9W<'a, REG> = crate::BitWriter<'a, REG, Adcpctl9>;
impl<'a, REG> Adcpctl9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input A9 disabled"]
    #[inline(always)]
    pub fn adcpctl9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl9::Adcpctl9_0)
    }
    #[doc = "ADC input A9 enabled"]
    #[inline(always)]
    pub fn adcpctl9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpctl9::Adcpctl9_1)
    }
}
#[doc = "RTC clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtccksel {
    #[doc = "0: SMCLK is selected"]
    RtcSmclk = 0,
    #[doc = "1: ACLK is selected"]
    RtcAclk = 1,
}
impl From<Rtccksel> for bool {
    #[inline(always)]
    fn from(variant: Rtccksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCKSEL` reader - RTC clock selection"]
pub type RtcckselR = crate::BitReader<Rtccksel>;
impl RtcckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtccksel {
        match self.bits {
            false => Rtccksel::RtcSmclk,
            true => Rtccksel::RtcAclk,
        }
    }
    #[doc = "SMCLK is selected"]
    #[inline(always)]
    pub fn is_rtc_smclk(&self) -> bool {
        *self == Rtccksel::RtcSmclk
    }
    #[doc = "ACLK is selected"]
    #[inline(always)]
    pub fn is_rtc_aclk(&self) -> bool {
        *self == Rtccksel::RtcAclk
    }
}
#[doc = "Field `RTCCKSEL` writer - RTC clock selection"]
pub type RtcckselW<'a, REG> = crate::BitWriter<'a, REG, Rtccksel>;
impl<'a, REG> RtcckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMCLK is selected"]
    #[inline(always)]
    pub fn rtc_smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccksel::RtcSmclk)
    }
    #[doc = "ACLK is selected"]
    #[inline(always)]
    pub fn rtc_aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccksel::RtcAclk)
    }
}
#[doc = "eUSCIB Remapping source selection , please refer to device specific for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscibrmp {
    #[doc = "0: P1.x is selected, please refer to device specific for details"]
    Uscibrmp0 = 0,
    #[doc = "1: other port is selected, please refer to device specific for details"]
    Uscibrmp1 = 1,
}
impl From<Uscibrmp> for bool {
    #[inline(always)]
    fn from(variant: Uscibrmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USCIBRMP` reader - eUSCIB Remapping source selection , please refer to device specific for details"]
pub type UscibrmpR = crate::BitReader<Uscibrmp>;
impl UscibrmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uscibrmp {
        match self.bits {
            false => Uscibrmp::Uscibrmp0,
            true => Uscibrmp::Uscibrmp1,
        }
    }
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn is_uscibrmp_0(&self) -> bool {
        *self == Uscibrmp::Uscibrmp0
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn is_uscibrmp_1(&self) -> bool {
        *self == Uscibrmp::Uscibrmp1
    }
}
#[doc = "Field `USCIBRMP` writer - eUSCIB Remapping source selection , please refer to device specific for details"]
pub type UscibrmpW<'a, REG> = crate::BitWriter<'a, REG, Uscibrmp>;
impl<'a, REG> UscibrmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscibrmp::Uscibrmp0)
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscibrmp::Uscibrmp1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&self) -> Adcpctl0R {
        Adcpctl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&self) -> Adcpctl1R {
        Adcpctl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&self) -> Adcpctl2R {
        Adcpctl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&self) -> Adcpctl3R {
        Adcpctl3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&self) -> Adcpctl4R {
        Adcpctl4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&self) -> Adcpctl5R {
        Adcpctl5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&self) -> Adcpctl6R {
        Adcpctl6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&self) -> Adcpctl7R {
        Adcpctl7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&self) -> Adcpctl8R {
        Adcpctl8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&self) -> Adcpctl9R {
        Adcpctl9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&self) -> RtcckselR {
        RtcckselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - eUSCIB Remapping source selection , please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp(&self) -> UscibrmpR {
        UscibrmpR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&mut self) -> Adcpctl0W<'_, Syscfg2Spec> {
        Adcpctl0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&mut self) -> Adcpctl1W<'_, Syscfg2Spec> {
        Adcpctl1W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&mut self) -> Adcpctl2W<'_, Syscfg2Spec> {
        Adcpctl2W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&mut self) -> Adcpctl3W<'_, Syscfg2Spec> {
        Adcpctl3W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&mut self) -> Adcpctl4W<'_, Syscfg2Spec> {
        Adcpctl4W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&mut self) -> Adcpctl5W<'_, Syscfg2Spec> {
        Adcpctl5W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&mut self) -> Adcpctl6W<'_, Syscfg2Spec> {
        Adcpctl6W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&mut self) -> Adcpctl7W<'_, Syscfg2Spec> {
        Adcpctl7W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&mut self) -> Adcpctl8W<'_, Syscfg2Spec> {
        Adcpctl8W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&mut self) -> Adcpctl9W<'_, Syscfg2Spec> {
        Adcpctl9W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&mut self) -> RtcckselW<'_, Syscfg2Spec> {
        RtcckselW::new(self, 10)
    }
    #[doc = "Bit 11 - eUSCIB Remapping source selection , please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp(&mut self) -> UscibrmpW<'_, Syscfg2Spec> {
        UscibrmpW::new(self, 11)
    }
}
#[doc = "System Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg2Spec;
impl crate::RegisterSpec for Syscfg2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg2::R`](R) reader structure"]
impl crate::Readable for Syscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg2::W`](W) writer structure"]
impl crate::Writable for Syscfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG2 to value 0"]
impl crate::Resettable for Syscfg2Spec {}
