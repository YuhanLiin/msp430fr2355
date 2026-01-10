#[doc = "Register `UCB1IFG` reader"]
pub type R = crate::R<Ucb1ifgSpec>;
#[doc = "Register `UCB1IFG` writer"]
pub type W = crate::W<Ucb1ifgSpec>;
#[doc = "eUSCI_B receive interrupt flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxifg0 {
    #[doc = "0: No interrupt pending"]
    Ucrxifg0_0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucrxifg0_1 = 1,
}
impl From<Ucrxifg0> for bool {
    #[inline(always)]
    fn from(variant: Ucrxifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG0` reader - eUSCI_B receive interrupt flag 0"]
pub type Ucrxifg0R = crate::BitReader<Ucrxifg0>;
impl Ucrxifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxifg0 {
        match self.bits {
            false => Ucrxifg0::Ucrxifg0_0,
            true => Ucrxifg0::Ucrxifg0_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg0_0(&self) -> bool {
        *self == Ucrxifg0::Ucrxifg0_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg0_1(&self) -> bool {
        *self == Ucrxifg0::Ucrxifg0_1
    }
}
#[doc = "Field `UCRXIFG0` writer - eUSCI_B receive interrupt flag 0"]
pub type Ucrxifg0W<'a, REG> = crate::BitWriter<'a, REG, Ucrxifg0>;
impl<'a, REG> Ucrxifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg0::Ucrxifg0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg0::Ucrxifg0_1)
    }
}
#[doc = "eUSCI_B transmit interrupt flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxifg0 {
    #[doc = "0: No interrupt pending"]
    Uctxifg0_0 = 0,
    #[doc = "1: Interrupt pending"]
    Uctxifg0_1 = 1,
}
impl From<Uctxifg0> for bool {
    #[inline(always)]
    fn from(variant: Uctxifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG0` reader - eUSCI_B transmit interrupt flag 0"]
pub type Uctxifg0R = crate::BitReader<Uctxifg0>;
impl Uctxifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxifg0 {
        match self.bits {
            false => Uctxifg0::Uctxifg0_0,
            true => Uctxifg0::Uctxifg0_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg0_0(&self) -> bool {
        *self == Uctxifg0::Uctxifg0_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg0_1(&self) -> bool {
        *self == Uctxifg0::Uctxifg0_1
    }
}
#[doc = "Field `UCTXIFG0` writer - eUSCI_B transmit interrupt flag 0"]
pub type Uctxifg0W<'a, REG> = crate::BitWriter<'a, REG, Uctxifg0>;
impl<'a, REG> Uctxifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg0::Uctxifg0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg0::Uctxifg0_1)
    }
}
#[doc = "START condition interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsttifg {
    #[doc = "0: No interrupt pending"]
    Ucsttifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucsttifg1 = 1,
}
impl From<Ucsttifg> for bool {
    #[inline(always)]
    fn from(variant: Ucsttifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTTIFG` reader - START condition interrupt flag"]
pub type UcsttifgR = crate::BitReader<Ucsttifg>;
impl UcsttifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsttifg {
        match self.bits {
            false => Ucsttifg::Ucsttifg0,
            true => Ucsttifg::Ucsttifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucsttifg_0(&self) -> bool {
        *self == Ucsttifg::Ucsttifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucsttifg_1(&self) -> bool {
        *self == Ucsttifg::Ucsttifg1
    }
}
#[doc = "Field `UCSTTIFG` writer - START condition interrupt flag"]
pub type UcsttifgW<'a, REG> = crate::BitWriter<'a, REG, Ucsttifg>;
impl<'a, REG> UcsttifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttifg::Ucsttifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttifg::Ucsttifg1)
    }
}
#[doc = "STOP condition interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucstpifg {
    #[doc = "0: No interrupt pending"]
    Ucstpifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucstpifg1 = 1,
}
impl From<Ucstpifg> for bool {
    #[inline(always)]
    fn from(variant: Ucstpifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTPIFG` reader - STOP condition interrupt flag"]
pub type UcstpifgR = crate::BitReader<Ucstpifg>;
impl UcstpifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucstpifg {
        match self.bits {
            false => Ucstpifg::Ucstpifg0,
            true => Ucstpifg::Ucstpifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucstpifg_0(&self) -> bool {
        *self == Ucstpifg::Ucstpifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucstpifg_1(&self) -> bool {
        *self == Ucstpifg::Ucstpifg1
    }
}
#[doc = "Field `UCSTPIFG` writer - STOP condition interrupt flag"]
pub type UcstpifgW<'a, REG> = crate::BitWriter<'a, REG, Ucstpifg>;
impl<'a, REG> UcstpifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucstpifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstpifg::Ucstpifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucstpifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstpifg::Ucstpifg1)
    }
}
#[doc = "Arbitration lost interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucalifg {
    #[doc = "0: No interrupt pending"]
    Ucalifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucalifg1 = 1,
}
impl From<Ucalifg> for bool {
    #[inline(always)]
    fn from(variant: Ucalifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCALIFG` reader - Arbitration lost interrupt flag"]
pub type UcalifgR = crate::BitReader<Ucalifg>;
impl UcalifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucalifg {
        match self.bits {
            false => Ucalifg::Ucalifg0,
            true => Ucalifg::Ucalifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucalifg_0(&self) -> bool {
        *self == Ucalifg::Ucalifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucalifg_1(&self) -> bool {
        *self == Ucalifg::Ucalifg1
    }
}
#[doc = "Field `UCALIFG` writer - Arbitration lost interrupt flag"]
pub type UcalifgW<'a, REG> = crate::BitWriter<'a, REG, Ucalifg>;
impl<'a, REG> UcalifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucalifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucalifg::Ucalifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucalifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucalifg::Ucalifg1)
    }
}
#[doc = "Not-acknowledge received interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucnackifg {
    #[doc = "0: No interrupt pending"]
    Ucnackifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucnackifg1 = 1,
}
impl From<Ucnackifg> for bool {
    #[inline(always)]
    fn from(variant: Ucnackifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCNACKIFG` reader - Not-acknowledge received interrupt flag"]
pub type UcnackifgR = crate::BitReader<Ucnackifg>;
impl UcnackifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucnackifg {
        match self.bits {
            false => Ucnackifg::Ucnackifg0,
            true => Ucnackifg::Ucnackifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucnackifg_0(&self) -> bool {
        *self == Ucnackifg::Ucnackifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucnackifg_1(&self) -> bool {
        *self == Ucnackifg::Ucnackifg1
    }
}
#[doc = "Field `UCNACKIFG` writer - Not-acknowledge received interrupt flag"]
pub type UcnackifgW<'a, REG> = crate::BitWriter<'a, REG, Ucnackifg>;
impl<'a, REG> UcnackifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucnackifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucnackifg::Ucnackifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucnackifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucnackifg::Ucnackifg1)
    }
}
#[doc = "Byte counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbcntifg {
    #[doc = "0: No interrupt pending"]
    Ucbcntifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucbcntifg1 = 1,
}
impl From<Ucbcntifg> for bool {
    #[inline(always)]
    fn from(variant: Ucbcntifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBCNTIFG` reader - Byte counter interrupt flag"]
pub type UcbcntifgR = crate::BitReader<Ucbcntifg>;
impl UcbcntifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbcntifg {
        match self.bits {
            false => Ucbcntifg::Ucbcntifg0,
            true => Ucbcntifg::Ucbcntifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucbcntifg_0(&self) -> bool {
        *self == Ucbcntifg::Ucbcntifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucbcntifg_1(&self) -> bool {
        *self == Ucbcntifg::Ucbcntifg1
    }
}
#[doc = "Field `UCBCNTIFG` writer - Byte counter interrupt flag"]
pub type UcbcntifgW<'a, REG> = crate::BitWriter<'a, REG, Ucbcntifg>;
impl<'a, REG> UcbcntifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucbcntifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbcntifg::Ucbcntifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucbcntifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbcntifg::Ucbcntifg1)
    }
}
#[doc = "Clock low timeout interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uccltoifg {
    #[doc = "0: No interrupt pending"]
    Uccltoifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Uccltoifg1 = 1,
}
impl From<Uccltoifg> for bool {
    #[inline(always)]
    fn from(variant: Uccltoifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCCLTOIFG` reader - Clock low timeout interrupt flag"]
pub type UccltoifgR = crate::BitReader<Uccltoifg>;
impl UccltoifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uccltoifg {
        match self.bits {
            false => Uccltoifg::Uccltoifg0,
            true => Uccltoifg::Uccltoifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uccltoifg_0(&self) -> bool {
        *self == Uccltoifg::Uccltoifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_uccltoifg_1(&self) -> bool {
        *self == Uccltoifg::Uccltoifg1
    }
}
#[doc = "Field `UCCLTOIFG` writer - Clock low timeout interrupt flag"]
pub type UccltoifgW<'a, REG> = crate::BitWriter<'a, REG, Uccltoifg>;
impl<'a, REG> UccltoifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uccltoifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uccltoifg::Uccltoifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uccltoifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uccltoifg::Uccltoifg1)
    }
}
#[doc = "eUSCI_B receive interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxifg1 {
    #[doc = "0: No interrupt pending"]
    Ucrxifg1_0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucrxifg1_1 = 1,
}
impl From<Ucrxifg1> for bool {
    #[inline(always)]
    fn from(variant: Ucrxifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG1` reader - eUSCI_B receive interrupt flag 1"]
pub type Ucrxifg1R = crate::BitReader<Ucrxifg1>;
impl Ucrxifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxifg1 {
        match self.bits {
            false => Ucrxifg1::Ucrxifg1_0,
            true => Ucrxifg1::Ucrxifg1_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg1_0(&self) -> bool {
        *self == Ucrxifg1::Ucrxifg1_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg1_1(&self) -> bool {
        *self == Ucrxifg1::Ucrxifg1_1
    }
}
#[doc = "Field `UCRXIFG1` writer - eUSCI_B receive interrupt flag 1"]
pub type Ucrxifg1W<'a, REG> = crate::BitWriter<'a, REG, Ucrxifg1>;
impl<'a, REG> Ucrxifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg1::Ucrxifg1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg1::Ucrxifg1_1)
    }
}
#[doc = "eUSCI_B transmit interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxifg1 {
    #[doc = "0: No interrupt pending"]
    Uctxifg1_0 = 0,
    #[doc = "1: Interrupt pending"]
    Uctxifg1_1 = 1,
}
impl From<Uctxifg1> for bool {
    #[inline(always)]
    fn from(variant: Uctxifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG1` reader - eUSCI_B transmit interrupt flag 1"]
pub type Uctxifg1R = crate::BitReader<Uctxifg1>;
impl Uctxifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxifg1 {
        match self.bits {
            false => Uctxifg1::Uctxifg1_0,
            true => Uctxifg1::Uctxifg1_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg1_0(&self) -> bool {
        *self == Uctxifg1::Uctxifg1_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg1_1(&self) -> bool {
        *self == Uctxifg1::Uctxifg1_1
    }
}
#[doc = "Field `UCTXIFG1` writer - eUSCI_B transmit interrupt flag 1"]
pub type Uctxifg1W<'a, REG> = crate::BitWriter<'a, REG, Uctxifg1>;
impl<'a, REG> Uctxifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg1::Uctxifg1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg1::Uctxifg1_1)
    }
}
#[doc = "eUSCI_B receive interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxifg2 {
    #[doc = "0: No interrupt pending"]
    Ucrxifg2_0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucrxifg2_1 = 1,
}
impl From<Ucrxifg2> for bool {
    #[inline(always)]
    fn from(variant: Ucrxifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG2` reader - eUSCI_B receive interrupt flag 2"]
pub type Ucrxifg2R = crate::BitReader<Ucrxifg2>;
impl Ucrxifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxifg2 {
        match self.bits {
            false => Ucrxifg2::Ucrxifg2_0,
            true => Ucrxifg2::Ucrxifg2_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg2_0(&self) -> bool {
        *self == Ucrxifg2::Ucrxifg2_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg2_1(&self) -> bool {
        *self == Ucrxifg2::Ucrxifg2_1
    }
}
#[doc = "Field `UCRXIFG2` writer - eUSCI_B receive interrupt flag 2"]
pub type Ucrxifg2W<'a, REG> = crate::BitWriter<'a, REG, Ucrxifg2>;
impl<'a, REG> Ucrxifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg2::Ucrxifg2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg2::Ucrxifg2_1)
    }
}
#[doc = "eUSCI_B transmit interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxifg2 {
    #[doc = "0: No interrupt pending"]
    Uctxifg2_0 = 0,
    #[doc = "1: Interrupt pending"]
    Uctxifg2_1 = 1,
}
impl From<Uctxifg2> for bool {
    #[inline(always)]
    fn from(variant: Uctxifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG2` reader - eUSCI_B transmit interrupt flag 2"]
pub type Uctxifg2R = crate::BitReader<Uctxifg2>;
impl Uctxifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxifg2 {
        match self.bits {
            false => Uctxifg2::Uctxifg2_0,
            true => Uctxifg2::Uctxifg2_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg2_0(&self) -> bool {
        *self == Uctxifg2::Uctxifg2_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg2_1(&self) -> bool {
        *self == Uctxifg2::Uctxifg2_1
    }
}
#[doc = "Field `UCTXIFG2` writer - eUSCI_B transmit interrupt flag 2"]
pub type Uctxifg2W<'a, REG> = crate::BitWriter<'a, REG, Uctxifg2>;
impl<'a, REG> Uctxifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg2::Uctxifg2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg2::Uctxifg2_1)
    }
}
#[doc = "eUSCI_B receive interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxifg3 {
    #[doc = "0: No interrupt pending"]
    Ucrxifg3_0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucrxifg3_1 = 1,
}
impl From<Ucrxifg3> for bool {
    #[inline(always)]
    fn from(variant: Ucrxifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG3` reader - eUSCI_B receive interrupt flag 3"]
pub type Ucrxifg3R = crate::BitReader<Ucrxifg3>;
impl Ucrxifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxifg3 {
        match self.bits {
            false => Ucrxifg3::Ucrxifg3_0,
            true => Ucrxifg3::Ucrxifg3_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg3_0(&self) -> bool {
        *self == Ucrxifg3::Ucrxifg3_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg3_1(&self) -> bool {
        *self == Ucrxifg3::Ucrxifg3_1
    }
}
#[doc = "Field `UCRXIFG3` writer - eUSCI_B receive interrupt flag 3"]
pub type Ucrxifg3W<'a, REG> = crate::BitWriter<'a, REG, Ucrxifg3>;
impl<'a, REG> Ucrxifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg3::Ucrxifg3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg3::Ucrxifg3_1)
    }
}
#[doc = "eUSCI_B transmit interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxifg3 {
    #[doc = "0: No interrupt pending"]
    Uctxifg3_0 = 0,
    #[doc = "1: Interrupt pending"]
    Uctxifg3_1 = 1,
}
impl From<Uctxifg3> for bool {
    #[inline(always)]
    fn from(variant: Uctxifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG3` reader - eUSCI_B transmit interrupt flag 3"]
pub type Uctxifg3R = crate::BitReader<Uctxifg3>;
impl Uctxifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxifg3 {
        match self.bits {
            false => Uctxifg3::Uctxifg3_0,
            true => Uctxifg3::Uctxifg3_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg3_0(&self) -> bool {
        *self == Uctxifg3::Uctxifg3_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg3_1(&self) -> bool {
        *self == Uctxifg3::Uctxifg3_1
    }
}
#[doc = "Field `UCTXIFG3` writer - eUSCI_B transmit interrupt flag 3"]
pub type Uctxifg3W<'a, REG> = crate::BitWriter<'a, REG, Uctxifg3>;
impl<'a, REG> Uctxifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg3::Uctxifg3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg3::Uctxifg3_1)
    }
}
#[doc = "Bit position 9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbit9ifg {
    #[doc = "0: No interrupt pending"]
    Ucbit9ifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucbit9ifg1 = 1,
}
impl From<Ucbit9ifg> for bool {
    #[inline(always)]
    fn from(variant: Ucbit9ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBIT9IFG` reader - Bit position 9 interrupt flag"]
pub type Ucbit9ifgR = crate::BitReader<Ucbit9ifg>;
impl Ucbit9ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbit9ifg {
        match self.bits {
            false => Ucbit9ifg::Ucbit9ifg0,
            true => Ucbit9ifg::Ucbit9ifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucbit9ifg_0(&self) -> bool {
        *self == Ucbit9ifg::Ucbit9ifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucbit9ifg_1(&self) -> bool {
        *self == Ucbit9ifg::Ucbit9ifg1
    }
}
#[doc = "Field `UCBIT9IFG` writer - Bit position 9 interrupt flag"]
pub type Ucbit9ifgW<'a, REG> = crate::BitWriter<'a, REG, Ucbit9ifg>;
impl<'a, REG> Ucbit9ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucbit9ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbit9ifg::Ucbit9ifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucbit9ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbit9ifg::Ucbit9ifg1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCI_B receive interrupt flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&self) -> Ucrxifg0R {
        Ucrxifg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - eUSCI_B transmit interrupt flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&self) -> Uctxifg0R {
        Uctxifg0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START condition interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UcsttifgR {
        UcsttifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP condition interrupt flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UcstpifgR {
        UcstpifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UcalifgR {
        UcalifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Not-acknowledge received interrupt flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UcnackifgR {
        UcnackifgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&self) -> UcbcntifgR {
        UcbcntifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt flag"]
    #[inline(always)]
    pub fn uccltoifg(&self) -> UccltoifgR {
        UccltoifgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - eUSCI_B receive interrupt flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&self) -> Ucrxifg1R {
        Ucrxifg1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - eUSCI_B transmit interrupt flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&self) -> Uctxifg1R {
        Uctxifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - eUSCI_B receive interrupt flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&self) -> Ucrxifg2R {
        Ucrxifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - eUSCI_B transmit interrupt flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&self) -> Uctxifg2R {
        Uctxifg2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - eUSCI_B receive interrupt flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&self) -> Ucrxifg3R {
        Ucrxifg3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - eUSCI_B transmit interrupt flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&self) -> Uctxifg3R {
        Uctxifg3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt flag"]
    #[inline(always)]
    pub fn ucbit9ifg(&self) -> Ucbit9ifgR {
        Ucbit9ifgR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCI_B receive interrupt flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&mut self) -> Ucrxifg0W<'_, Ucb1ifgSpec> {
        Ucrxifg0W::new(self, 0)
    }
    #[doc = "Bit 1 - eUSCI_B transmit interrupt flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&mut self) -> Uctxifg0W<'_, Ucb1ifgSpec> {
        Uctxifg0W::new(self, 1)
    }
    #[doc = "Bit 2 - START condition interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UcsttifgW<'_, Ucb1ifgSpec> {
        UcsttifgW::new(self, 2)
    }
    #[doc = "Bit 3 - STOP condition interrupt flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UcstpifgW<'_, Ucb1ifgSpec> {
        UcstpifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UcalifgW<'_, Ucb1ifgSpec> {
        UcalifgW::new(self, 4)
    }
    #[doc = "Bit 5 - Not-acknowledge received interrupt flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UcnackifgW<'_, Ucb1ifgSpec> {
        UcnackifgW::new(self, 5)
    }
    #[doc = "Bit 6 - Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&mut self) -> UcbcntifgW<'_, Ucb1ifgSpec> {
        UcbcntifgW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt flag"]
    #[inline(always)]
    pub fn uccltoifg(&mut self) -> UccltoifgW<'_, Ucb1ifgSpec> {
        UccltoifgW::new(self, 7)
    }
    #[doc = "Bit 8 - eUSCI_B receive interrupt flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&mut self) -> Ucrxifg1W<'_, Ucb1ifgSpec> {
        Ucrxifg1W::new(self, 8)
    }
    #[doc = "Bit 9 - eUSCI_B transmit interrupt flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&mut self) -> Uctxifg1W<'_, Ucb1ifgSpec> {
        Uctxifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - eUSCI_B receive interrupt flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&mut self) -> Ucrxifg2W<'_, Ucb1ifgSpec> {
        Ucrxifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - eUSCI_B transmit interrupt flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&mut self) -> Uctxifg2W<'_, Ucb1ifgSpec> {
        Uctxifg2W::new(self, 11)
    }
    #[doc = "Bit 12 - eUSCI_B receive interrupt flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&mut self) -> Ucrxifg3W<'_, Ucb1ifgSpec> {
        Ucrxifg3W::new(self, 12)
    }
    #[doc = "Bit 13 - eUSCI_B transmit interrupt flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&mut self) -> Uctxifg3W<'_, Ucb1ifgSpec> {
        Uctxifg3W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt flag"]
    #[inline(always)]
    pub fn ucbit9ifg(&mut self) -> Ucbit9ifgW<'_, Ucb1ifgSpec> {
        Ucbit9ifgW::new(self, 14)
    }
}
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1ifgSpec;
impl crate::RegisterSpec for Ucb1ifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1ifg::R`](R) reader structure"]
impl crate::Readable for Ucb1ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1ifg::W`](W) writer structure"]
impl crate::Writable for Ucb1ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1IFG to value 0"]
impl crate::Resettable for Ucb1ifgSpec {}
