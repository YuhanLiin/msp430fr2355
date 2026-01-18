#[doc = "Register `UCB1IE` reader"]
pub type R = crate::R<Ucb1ieSpec>;
#[doc = "Register `UCB1IE` writer"]
pub type W = crate::W<Ucb1ieSpec>;
#[doc = "Receive interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxie0 {
    #[doc = "0: Interrupt disabled"]
    Ucrxie0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucrxie0_1 = 1,
}
impl From<Ucrxie0> for bool {
    #[inline(always)]
    fn from(variant: Ucrxie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIE0` reader - Receive interrupt enable 0"]
pub type Ucrxie0R = crate::BitReader<Ucrxie0>;
impl Ucrxie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxie0 {
        match self.bits {
            false => Ucrxie0::Ucrxie0_0,
            true => Ucrxie0::Ucrxie0_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucrxie0_0(&self) -> bool {
        *self == Ucrxie0::Ucrxie0_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucrxie0_1(&self) -> bool {
        *self == Ucrxie0::Ucrxie0_1
    }
}
#[doc = "Field `UCRXIE0` writer - Receive interrupt enable 0"]
pub type Ucrxie0W<'a, REG> = crate::BitWriter<'a, REG, Ucrxie0>;
impl<'a, REG> Ucrxie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie0::Ucrxie0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie0::Ucrxie0_1)
    }
}
#[doc = "Transmit interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxie0 {
    #[doc = "0: Interrupt disabled"]
    Uctxie0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Uctxie0_1 = 1,
}
impl From<Uctxie0> for bool {
    #[inline(always)]
    fn from(variant: Uctxie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIE0` reader - Transmit interrupt enable 0"]
pub type Uctxie0R = crate::BitReader<Uctxie0>;
impl Uctxie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxie0 {
        match self.bits {
            false => Uctxie0::Uctxie0_0,
            true => Uctxie0::Uctxie0_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_uctxie0_0(&self) -> bool {
        *self == Uctxie0::Uctxie0_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_uctxie0_1(&self) -> bool {
        *self == Uctxie0::Uctxie0_1
    }
}
#[doc = "Field `UCTXIE0` writer - Transmit interrupt enable 0"]
pub type Uctxie0W<'a, REG> = crate::BitWriter<'a, REG, Uctxie0>;
impl<'a, REG> Uctxie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie0::Uctxie0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie0::Uctxie0_1)
    }
}
#[doc = "START condition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsttie {
    #[doc = "0: Interrupt disabled"]
    Ucsttie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucsttie1 = 1,
}
impl From<Ucsttie> for bool {
    #[inline(always)]
    fn from(variant: Ucsttie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTTIE` reader - START condition interrupt enable"]
pub type UcsttieR = crate::BitReader<Ucsttie>;
impl UcsttieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsttie {
        match self.bits {
            false => Ucsttie::Ucsttie0,
            true => Ucsttie::Ucsttie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucsttie_0(&self) -> bool {
        *self == Ucsttie::Ucsttie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucsttie_1(&self) -> bool {
        *self == Ucsttie::Ucsttie1
    }
}
#[doc = "Field `UCSTTIE` writer - START condition interrupt enable"]
pub type UcsttieW<'a, REG> = crate::BitWriter<'a, REG, Ucsttie>;
impl<'a, REG> UcsttieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucsttie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttie::Ucsttie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucsttie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttie::Ucsttie1)
    }
}
#[doc = "STOP condition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucstpie {
    #[doc = "0: Interrupt disabled"]
    Ucstpie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucstpie1 = 1,
}
impl From<Ucstpie> for bool {
    #[inline(always)]
    fn from(variant: Ucstpie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTPIE` reader - STOP condition interrupt enable"]
pub type UcstpieR = crate::BitReader<Ucstpie>;
impl UcstpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucstpie {
        match self.bits {
            false => Ucstpie::Ucstpie0,
            true => Ucstpie::Ucstpie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucstpie_0(&self) -> bool {
        *self == Ucstpie::Ucstpie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucstpie_1(&self) -> bool {
        *self == Ucstpie::Ucstpie1
    }
}
#[doc = "Field `UCSTPIE` writer - STOP condition interrupt enable"]
pub type UcstpieW<'a, REG> = crate::BitWriter<'a, REG, Ucstpie>;
impl<'a, REG> UcstpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucstpie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstpie::Ucstpie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucstpie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstpie::Ucstpie1)
    }
}
#[doc = "Arbitration lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucalie {
    #[doc = "0: Interrupt disabled"]
    Ucalie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucalie1 = 1,
}
impl From<Ucalie> for bool {
    #[inline(always)]
    fn from(variant: Ucalie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCALIE` reader - Arbitration lost interrupt enable"]
pub type UcalieR = crate::BitReader<Ucalie>;
impl UcalieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucalie {
        match self.bits {
            false => Ucalie::Ucalie0,
            true => Ucalie::Ucalie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucalie_0(&self) -> bool {
        *self == Ucalie::Ucalie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucalie_1(&self) -> bool {
        *self == Ucalie::Ucalie1
    }
}
#[doc = "Field `UCALIE` writer - Arbitration lost interrupt enable"]
pub type UcalieW<'a, REG> = crate::BitWriter<'a, REG, Ucalie>;
impl<'a, REG> UcalieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucalie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucalie::Ucalie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucalie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucalie::Ucalie1)
    }
}
#[doc = "Not-acknowledge interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucnackie {
    #[doc = "0: Interrupt disabled"]
    Ucnackie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucnackie1 = 1,
}
impl From<Ucnackie> for bool {
    #[inline(always)]
    fn from(variant: Ucnackie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCNACKIE` reader - Not-acknowledge interrupt enable"]
pub type UcnackieR = crate::BitReader<Ucnackie>;
impl UcnackieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucnackie {
        match self.bits {
            false => Ucnackie::Ucnackie0,
            true => Ucnackie::Ucnackie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucnackie_0(&self) -> bool {
        *self == Ucnackie::Ucnackie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucnackie_1(&self) -> bool {
        *self == Ucnackie::Ucnackie1
    }
}
#[doc = "Field `UCNACKIE` writer - Not-acknowledge interrupt enable"]
pub type UcnackieW<'a, REG> = crate::BitWriter<'a, REG, Ucnackie>;
impl<'a, REG> UcnackieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucnackie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucnackie::Ucnackie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucnackie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucnackie::Ucnackie1)
    }
}
#[doc = "Byte counter interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbcntie {
    #[doc = "0: Interrupt disabled"]
    Ucbcntie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucbcntie1 = 1,
}
impl From<Ucbcntie> for bool {
    #[inline(always)]
    fn from(variant: Ucbcntie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBCNTIE` reader - Byte counter interrupt enable"]
pub type UcbcntieR = crate::BitReader<Ucbcntie>;
impl UcbcntieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbcntie {
        match self.bits {
            false => Ucbcntie::Ucbcntie0,
            true => Ucbcntie::Ucbcntie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucbcntie_0(&self) -> bool {
        *self == Ucbcntie::Ucbcntie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucbcntie_1(&self) -> bool {
        *self == Ucbcntie::Ucbcntie1
    }
}
#[doc = "Field `UCBCNTIE` writer - Byte counter interrupt enable"]
pub type UcbcntieW<'a, REG> = crate::BitWriter<'a, REG, Ucbcntie>;
impl<'a, REG> UcbcntieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucbcntie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbcntie::Ucbcntie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucbcntie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbcntie::Ucbcntie1)
    }
}
#[doc = "Clock low timeout interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uccltoie {
    #[doc = "0: Interrupt disabled"]
    Uccltoie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Uccltoie1 = 1,
}
impl From<Uccltoie> for bool {
    #[inline(always)]
    fn from(variant: Uccltoie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCCLTOIE` reader - Clock low timeout interrupt enable"]
pub type UccltoieR = crate::BitReader<Uccltoie>;
impl UccltoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uccltoie {
        match self.bits {
            false => Uccltoie::Uccltoie0,
            true => Uccltoie::Uccltoie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_uccltoie_0(&self) -> bool {
        *self == Uccltoie::Uccltoie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_uccltoie_1(&self) -> bool {
        *self == Uccltoie::Uccltoie1
    }
}
#[doc = "Field `UCCLTOIE` writer - Clock low timeout interrupt enable"]
pub type UccltoieW<'a, REG> = crate::BitWriter<'a, REG, Uccltoie>;
impl<'a, REG> UccltoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uccltoie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uccltoie::Uccltoie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uccltoie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uccltoie::Uccltoie1)
    }
}
#[doc = "Receive interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxie1 {
    #[doc = "0: Interrupt disabled"]
    Ucrxie1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucrxie1_1 = 1,
}
impl From<Ucrxie1> for bool {
    #[inline(always)]
    fn from(variant: Ucrxie1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIE1` reader - Receive interrupt enable 1"]
pub type Ucrxie1R = crate::BitReader<Ucrxie1>;
impl Ucrxie1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxie1 {
        match self.bits {
            false => Ucrxie1::Ucrxie1_0,
            true => Ucrxie1::Ucrxie1_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucrxie1_0(&self) -> bool {
        *self == Ucrxie1::Ucrxie1_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucrxie1_1(&self) -> bool {
        *self == Ucrxie1::Ucrxie1_1
    }
}
#[doc = "Field `UCRXIE1` writer - Receive interrupt enable 1"]
pub type Ucrxie1W<'a, REG> = crate::BitWriter<'a, REG, Ucrxie1>;
impl<'a, REG> Ucrxie1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie1::Ucrxie1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie1::Ucrxie1_1)
    }
}
#[doc = "Transmit interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxie1 {
    #[doc = "0: Interrupt disabled"]
    Uctxie1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Uctxie1_1 = 1,
}
impl From<Uctxie1> for bool {
    #[inline(always)]
    fn from(variant: Uctxie1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIE1` reader - Transmit interrupt enable 1"]
pub type Uctxie1R = crate::BitReader<Uctxie1>;
impl Uctxie1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxie1 {
        match self.bits {
            false => Uctxie1::Uctxie1_0,
            true => Uctxie1::Uctxie1_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_uctxie1_0(&self) -> bool {
        *self == Uctxie1::Uctxie1_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_uctxie1_1(&self) -> bool {
        *self == Uctxie1::Uctxie1_1
    }
}
#[doc = "Field `UCTXIE1` writer - Transmit interrupt enable 1"]
pub type Uctxie1W<'a, REG> = crate::BitWriter<'a, REG, Uctxie1>;
impl<'a, REG> Uctxie1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie1::Uctxie1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie1::Uctxie1_1)
    }
}
#[doc = "Receive interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxie2 {
    #[doc = "0: Interrupt disabled"]
    Ucrxie2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucrxie2_1 = 1,
}
impl From<Ucrxie2> for bool {
    #[inline(always)]
    fn from(variant: Ucrxie2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIE2` reader - Receive interrupt enable 2"]
pub type Ucrxie2R = crate::BitReader<Ucrxie2>;
impl Ucrxie2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxie2 {
        match self.bits {
            false => Ucrxie2::Ucrxie2_0,
            true => Ucrxie2::Ucrxie2_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucrxie2_0(&self) -> bool {
        *self == Ucrxie2::Ucrxie2_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucrxie2_1(&self) -> bool {
        *self == Ucrxie2::Ucrxie2_1
    }
}
#[doc = "Field `UCRXIE2` writer - Receive interrupt enable 2"]
pub type Ucrxie2W<'a, REG> = crate::BitWriter<'a, REG, Ucrxie2>;
impl<'a, REG> Ucrxie2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie2::Ucrxie2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie2::Ucrxie2_1)
    }
}
#[doc = "Transmit interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxie2 {
    #[doc = "0: Interrupt disabled"]
    Uctxie2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Uctxie2_1 = 1,
}
impl From<Uctxie2> for bool {
    #[inline(always)]
    fn from(variant: Uctxie2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIE2` reader - Transmit interrupt enable 2"]
pub type Uctxie2R = crate::BitReader<Uctxie2>;
impl Uctxie2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxie2 {
        match self.bits {
            false => Uctxie2::Uctxie2_0,
            true => Uctxie2::Uctxie2_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_uctxie2_0(&self) -> bool {
        *self == Uctxie2::Uctxie2_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_uctxie2_1(&self) -> bool {
        *self == Uctxie2::Uctxie2_1
    }
}
#[doc = "Field `UCTXIE2` writer - Transmit interrupt enable 2"]
pub type Uctxie2W<'a, REG> = crate::BitWriter<'a, REG, Uctxie2>;
impl<'a, REG> Uctxie2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie2::Uctxie2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie2::Uctxie2_1)
    }
}
#[doc = "Receive interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxie3 {
    #[doc = "0: Interrupt disabled"]
    Ucrxie3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucrxie3_1 = 1,
}
impl From<Ucrxie3> for bool {
    #[inline(always)]
    fn from(variant: Ucrxie3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIE3` reader - Receive interrupt enable 3"]
pub type Ucrxie3R = crate::BitReader<Ucrxie3>;
impl Ucrxie3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxie3 {
        match self.bits {
            false => Ucrxie3::Ucrxie3_0,
            true => Ucrxie3::Ucrxie3_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucrxie3_0(&self) -> bool {
        *self == Ucrxie3::Ucrxie3_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucrxie3_1(&self) -> bool {
        *self == Ucrxie3::Ucrxie3_1
    }
}
#[doc = "Field `UCRXIE3` writer - Receive interrupt enable 3"]
pub type Ucrxie3W<'a, REG> = crate::BitWriter<'a, REG, Ucrxie3>;
impl<'a, REG> Ucrxie3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie3::Ucrxie3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie3::Ucrxie3_1)
    }
}
#[doc = "Transmit interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxie3 {
    #[doc = "0: Interrupt disabled"]
    Uctxie3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Uctxie3_1 = 1,
}
impl From<Uctxie3> for bool {
    #[inline(always)]
    fn from(variant: Uctxie3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIE3` reader - Transmit interrupt enable 3"]
pub type Uctxie3R = crate::BitReader<Uctxie3>;
impl Uctxie3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxie3 {
        match self.bits {
            false => Uctxie3::Uctxie3_0,
            true => Uctxie3::Uctxie3_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_uctxie3_0(&self) -> bool {
        *self == Uctxie3::Uctxie3_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_uctxie3_1(&self) -> bool {
        *self == Uctxie3::Uctxie3_1
    }
}
#[doc = "Field `UCTXIE3` writer - Transmit interrupt enable 3"]
pub type Uctxie3W<'a, REG> = crate::BitWriter<'a, REG, Uctxie3>;
impl<'a, REG> Uctxie3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie3::Uctxie3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie3::Uctxie3_1)
    }
}
#[doc = "Bit position 9 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbit9ie {
    #[doc = "0: Interrupt disabled"]
    Ucbit9ie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucbit9ie1 = 1,
}
impl From<Ucbit9ie> for bool {
    #[inline(always)]
    fn from(variant: Ucbit9ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBIT9IE` reader - Bit position 9 interrupt enable"]
pub type Ucbit9ieR = crate::BitReader<Ucbit9ie>;
impl Ucbit9ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbit9ie {
        match self.bits {
            false => Ucbit9ie::Ucbit9ie0,
            true => Ucbit9ie::Ucbit9ie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucbit9ie_0(&self) -> bool {
        *self == Ucbit9ie::Ucbit9ie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucbit9ie_1(&self) -> bool {
        *self == Ucbit9ie::Ucbit9ie1
    }
}
#[doc = "Field `UCBIT9IE` writer - Bit position 9 interrupt enable"]
pub type Ucbit9ieW<'a, REG> = crate::BitWriter<'a, REG, Ucbit9ie>;
impl<'a, REG> Ucbit9ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucbit9ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbit9ie::Ucbit9ie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucbit9ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbit9ie::Ucbit9ie1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> Ucrxie0R {
        Ucrxie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> Uctxie0R {
        Uctxie0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UcsttieR {
        UcsttieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UcstpieR {
        UcstpieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UcalieR {
        UcalieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Not-acknowledge interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UcnackieR {
        UcnackieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Byte counter interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UcbcntieR {
        UcbcntieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UccltoieR {
        UccltoieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive interrupt enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> Ucrxie1R {
        Ucrxie1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit interrupt enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> Uctxie1R {
        Uctxie1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive interrupt enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> Ucrxie2R {
        Ucrxie2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit interrupt enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> Uctxie2R {
        Uctxie2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive interrupt enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> Ucrxie3R {
        Ucrxie3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit interrupt enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> Uctxie3R {
        Uctxie3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt enable"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> Ucbit9ieR {
        Ucbit9ieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&mut self) -> Ucrxie0W<'_, Ucb1ieSpec> {
        Ucrxie0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable 0"]
    #[inline(always)]
    pub fn uctxie0(&mut self) -> Uctxie0W<'_, Ucb1ieSpec> {
        Uctxie0W::new(self, 1)
    }
    #[doc = "Bit 2 - START condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UcsttieW<'_, Ucb1ieSpec> {
        UcsttieW::new(self, 2)
    }
    #[doc = "Bit 3 - STOP condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UcstpieW<'_, Ucb1ieSpec> {
        UcstpieW::new(self, 3)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UcalieW<'_, Ucb1ieSpec> {
        UcalieW::new(self, 4)
    }
    #[doc = "Bit 5 - Not-acknowledge interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UcnackieW<'_, Ucb1ieSpec> {
        UcnackieW::new(self, 5)
    }
    #[doc = "Bit 6 - Byte counter interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&mut self) -> UcbcntieW<'_, Ucb1ieSpec> {
        UcbcntieW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&mut self) -> UccltoieW<'_, Ucb1ieSpec> {
        UccltoieW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive interrupt enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&mut self) -> Ucrxie1W<'_, Ucb1ieSpec> {
        Ucrxie1W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit interrupt enable 1"]
    #[inline(always)]
    pub fn uctxie1(&mut self) -> Uctxie1W<'_, Ucb1ieSpec> {
        Uctxie1W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive interrupt enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&mut self) -> Ucrxie2W<'_, Ucb1ieSpec> {
        Ucrxie2W::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit interrupt enable 2"]
    #[inline(always)]
    pub fn uctxie2(&mut self) -> Uctxie2W<'_, Ucb1ieSpec> {
        Uctxie2W::new(self, 11)
    }
    #[doc = "Bit 12 - Receive interrupt enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&mut self) -> Ucrxie3W<'_, Ucb1ieSpec> {
        Ucrxie3W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit interrupt enable 3"]
    #[inline(always)]
    pub fn uctxie3(&mut self) -> Uctxie3W<'_, Ucb1ieSpec> {
        Uctxie3W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt enable"]
    #[inline(always)]
    pub fn ucbit9ie(&mut self) -> Ucbit9ieW<'_, Ucb1ieSpec> {
        Ucbit9ieW::new(self, 14)
    }
}
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1ieSpec;
impl crate::RegisterSpec for Ucb1ieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1ie::R`](R) reader structure"]
impl crate::Readable for Ucb1ieSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1ie::W`](W) writer structure"]
impl crate::Writable for Ucb1ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1IE to value 0"]
impl crate::Resettable for Ucb1ieSpec {}
