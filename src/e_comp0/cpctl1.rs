#[doc = "Register `CPCTL1` reader"]
pub type R = crate::R<Cpctl1Spec>;
#[doc = "Register `CPCTL1` writer"]
pub type W = crate::W<Cpctl1Spec>;
#[doc = "Field `CPOUT` reader - Comparator output value"]
pub type CpoutR = crate::BitReader;
#[doc = "Comparator output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpinv {
    #[doc = "0: Comparator output is non-inverted"]
    Cpinv0 = 0,
    #[doc = "1: Comparator output is inverted"]
    Cpinv1 = 1,
}
impl From<Cpinv> for bool {
    #[inline(always)]
    fn from(variant: Cpinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPINV` reader - Comparator output polarity"]
pub type CpinvR = crate::BitReader<Cpinv>;
impl CpinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpinv {
        match self.bits {
            false => Cpinv::Cpinv0,
            true => Cpinv::Cpinv1,
        }
    }
    #[doc = "Comparator output is non-inverted"]
    #[inline(always)]
    pub fn is_cpinv_0(&self) -> bool {
        *self == Cpinv::Cpinv0
    }
    #[doc = "Comparator output is inverted"]
    #[inline(always)]
    pub fn is_cpinv_1(&self) -> bool {
        *self == Cpinv::Cpinv1
    }
}
#[doc = "Field `CPINV` writer - Comparator output polarity"]
pub type CpinvW<'a, REG> = crate::BitWriter<'a, REG, Cpinv>;
impl<'a, REG> CpinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output is non-inverted"]
    #[inline(always)]
    pub fn cpinv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpinv::Cpinv0)
    }
    #[doc = "Comparator output is inverted"]
    #[inline(always)]
    pub fn cpinv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpinv::Cpinv1)
    }
}
#[doc = "Interrupt edge select for CEIIFG and CEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpies {
    #[doc = "0: Rising edge for CPIFG, falling edge for CPIIFG"]
    Cpies0 = 0,
    #[doc = "1: Falling edge for CPIFG, rising edge for CPIIFG"]
    Cpies1 = 1,
}
impl From<Cpies> for bool {
    #[inline(always)]
    fn from(variant: Cpies) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPIES` reader - Interrupt edge select for CEIIFG and CEIFG"]
pub type CpiesR = crate::BitReader<Cpies>;
impl CpiesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpies {
        match self.bits {
            false => Cpies::Cpies0,
            true => Cpies::Cpies1,
        }
    }
    #[doc = "Rising edge for CPIFG, falling edge for CPIIFG"]
    #[inline(always)]
    pub fn is_cpies_0(&self) -> bool {
        *self == Cpies::Cpies0
    }
    #[doc = "Falling edge for CPIFG, rising edge for CPIIFG"]
    #[inline(always)]
    pub fn is_cpies_1(&self) -> bool {
        *self == Cpies::Cpies1
    }
}
#[doc = "Field `CPIES` writer - Interrupt edge select for CEIIFG and CEIFG"]
pub type CpiesW<'a, REG> = crate::BitWriter<'a, REG, Cpies>;
impl<'a, REG> CpiesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge for CPIFG, falling edge for CPIIFG"]
    #[inline(always)]
    pub fn cpies_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpies::Cpies0)
    }
    #[doc = "Falling edge for CPIFG, rising edge for CPIIFG"]
    #[inline(always)]
    pub fn cpies_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpies::Cpies1)
    }
}
#[doc = "Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpflt {
    #[doc = "0: Comparator output is not filtered"]
    Cpflt0 = 0,
    #[doc = "1: Comparator output is filtered"]
    Cpflt1 = 1,
}
impl From<Cpflt> for bool {
    #[inline(always)]
    fn from(variant: Cpflt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPFLT` reader - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
pub type CpfltR = crate::BitReader<Cpflt>;
impl CpfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpflt {
        match self.bits {
            false => Cpflt::Cpflt0,
            true => Cpflt::Cpflt1,
        }
    }
    #[doc = "Comparator output is not filtered"]
    #[inline(always)]
    pub fn is_cpflt_0(&self) -> bool {
        *self == Cpflt::Cpflt0
    }
    #[doc = "Comparator output is filtered"]
    #[inline(always)]
    pub fn is_cpflt_1(&self) -> bool {
        *self == Cpflt::Cpflt1
    }
}
#[doc = "Field `CPFLT` writer - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
pub type CpfltW<'a, REG> = crate::BitWriter<'a, REG, Cpflt>;
impl<'a, REG> CpfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output is not filtered"]
    #[inline(always)]
    pub fn cpflt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpflt::Cpflt0)
    }
    #[doc = "Comparator output is filtered"]
    #[inline(always)]
    pub fn cpflt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpflt::Cpflt1)
    }
}
#[doc = "Analog Filter Delay selection. These bits are used to select the analog filter delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpfltdly {
    #[doc = "0: Typical filter delay of 450ns"]
    Cpfltdly0 = 0,
    #[doc = "1: Typical filter delay of 900ns"]
    Cpfltdly1 = 1,
    #[doc = "2: Typical filter delay of 1800ns"]
    Cpfltdly2 = 2,
    #[doc = "3: Typical filter delay of 3600ns"]
    Cpfltdly3 = 3,
}
impl From<Cpfltdly> for u8 {
    #[inline(always)]
    fn from(variant: Cpfltdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpfltdly {
    type Ux = u8;
}
impl crate::IsEnum for Cpfltdly {}
#[doc = "Field `CPFLTDLY` reader - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
pub type CpfltdlyR = crate::FieldReader<Cpfltdly>;
impl CpfltdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpfltdly {
        match self.bits {
            0 => Cpfltdly::Cpfltdly0,
            1 => Cpfltdly::Cpfltdly1,
            2 => Cpfltdly::Cpfltdly2,
            3 => Cpfltdly::Cpfltdly3,
            _ => unreachable!(),
        }
    }
    #[doc = "Typical filter delay of 450ns"]
    #[inline(always)]
    pub fn is_cpfltdly_0(&self) -> bool {
        *self == Cpfltdly::Cpfltdly0
    }
    #[doc = "Typical filter delay of 900ns"]
    #[inline(always)]
    pub fn is_cpfltdly_1(&self) -> bool {
        *self == Cpfltdly::Cpfltdly1
    }
    #[doc = "Typical filter delay of 1800ns"]
    #[inline(always)]
    pub fn is_cpfltdly_2(&self) -> bool {
        *self == Cpfltdly::Cpfltdly2
    }
    #[doc = "Typical filter delay of 3600ns"]
    #[inline(always)]
    pub fn is_cpfltdly_3(&self) -> bool {
        *self == Cpfltdly::Cpfltdly3
    }
}
#[doc = "Field `CPFLTDLY` writer - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
pub type CpfltdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpfltdly, crate::Safe>;
impl<'a, REG> CpfltdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Typical filter delay of 450ns"]
    #[inline(always)]
    pub fn cpfltdly_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpfltdly::Cpfltdly0)
    }
    #[doc = "Typical filter delay of 900ns"]
    #[inline(always)]
    pub fn cpfltdly_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpfltdly::Cpfltdly1)
    }
    #[doc = "Typical filter delay of 1800ns"]
    #[inline(always)]
    pub fn cpfltdly_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpfltdly::Cpfltdly2)
    }
    #[doc = "Typical filter delay of 3600ns"]
    #[inline(always)]
    pub fn cpfltdly_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpfltdly::Cpfltdly3)
    }
}
#[doc = "Power mode selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpmsel {
    #[doc = "0: High-power & High speed mode (500nA)"]
    Cpmsel0 = 0,
    #[doc = "1: Low-power & Low speed mode (10nA)"]
    Cpmsel1 = 1,
}
impl From<Cpmsel> for bool {
    #[inline(always)]
    fn from(variant: Cpmsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPMSEL` reader - Power mode selection."]
pub type CpmselR = crate::BitReader<Cpmsel>;
impl CpmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpmsel {
        match self.bits {
            false => Cpmsel::Cpmsel0,
            true => Cpmsel::Cpmsel1,
        }
    }
    #[doc = "High-power & High speed mode (500nA)"]
    #[inline(always)]
    pub fn is_cpmsel_0(&self) -> bool {
        *self == Cpmsel::Cpmsel0
    }
    #[doc = "Low-power & Low speed mode (10nA)"]
    #[inline(always)]
    pub fn is_cpmsel_1(&self) -> bool {
        *self == Cpmsel::Cpmsel1
    }
}
#[doc = "Field `CPMSEL` writer - Power mode selection."]
pub type CpmselW<'a, REG> = crate::BitWriter<'a, REG, Cpmsel>;
impl<'a, REG> CpmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High-power & High speed mode (500nA)"]
    #[inline(always)]
    pub fn cpmsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpmsel::Cpmsel0)
    }
    #[doc = "Low-power & Low speed mode (10nA)"]
    #[inline(always)]
    pub fn cpmsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpmsel::Cpmsel1)
    }
}
#[doc = "Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpen {
    #[doc = "0: Comparator is disabled"]
    Cpen0 = 0,
    #[doc = "1: Comparator is enabled"]
    Cpen1 = 1,
}
impl From<Cpen> for bool {
    #[inline(always)]
    fn from(variant: Cpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPEN` reader - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
pub type CpenR = crate::BitReader<Cpen>;
impl CpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpen {
        match self.bits {
            false => Cpen::Cpen0,
            true => Cpen::Cpen1,
        }
    }
    #[doc = "Comparator is disabled"]
    #[inline(always)]
    pub fn is_cpen_0(&self) -> bool {
        *self == Cpen::Cpen0
    }
    #[doc = "Comparator is enabled"]
    #[inline(always)]
    pub fn is_cpen_1(&self) -> bool {
        *self == Cpen::Cpen1
    }
}
#[doc = "Field `CPEN` writer - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
pub type CpenW<'a, REG> = crate::BitWriter<'a, REG, Cpen>;
impl<'a, REG> CpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator is disabled"]
    #[inline(always)]
    pub fn cpen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpen::Cpen0)
    }
    #[doc = "Comparator is enabled"]
    #[inline(always)]
    pub fn cpen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpen::Cpen1)
    }
}
#[doc = "Programable Hysteresis mode. These bits are used to select the Hysteresis mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cphsel {
    #[doc = "0: disable"]
    Cphsel0 = 0,
    #[doc = "1: 10mV"]
    Cphsel1 = 1,
    #[doc = "2: 20mV"]
    Cphsel2 = 2,
    #[doc = "3: 30mV"]
    Cphsel3 = 3,
}
impl From<Cphsel> for u8 {
    #[inline(always)]
    fn from(variant: Cphsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cphsel {
    type Ux = u8;
}
impl crate::IsEnum for Cphsel {}
#[doc = "Field `CPHSEL` reader - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
pub type CphselR = crate::FieldReader<Cphsel>;
impl CphselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphsel {
        match self.bits {
            0 => Cphsel::Cphsel0,
            1 => Cphsel::Cphsel1,
            2 => Cphsel::Cphsel2,
            3 => Cphsel::Cphsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_cphsel_0(&self) -> bool {
        *self == Cphsel::Cphsel0
    }
    #[doc = "10mV"]
    #[inline(always)]
    pub fn is_cphsel_1(&self) -> bool {
        *self == Cphsel::Cphsel1
    }
    #[doc = "20mV"]
    #[inline(always)]
    pub fn is_cphsel_2(&self) -> bool {
        *self == Cphsel::Cphsel2
    }
    #[doc = "30mV"]
    #[inline(always)]
    pub fn is_cphsel_3(&self) -> bool {
        *self == Cphsel::Cphsel3
    }
}
#[doc = "Field `CPHSEL` writer - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
pub type CphselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cphsel, crate::Safe>;
impl<'a, REG> CphselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn cphsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphsel::Cphsel0)
    }
    #[doc = "10mV"]
    #[inline(always)]
    pub fn cphsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphsel::Cphsel1)
    }
    #[doc = "20mV"]
    #[inline(always)]
    pub fn cphsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cphsel::Cphsel2)
    }
    #[doc = "30mV"]
    #[inline(always)]
    pub fn cphsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cphsel::Cphsel3)
    }
}
#[doc = "Comparator interrupt output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpie {
    #[doc = "0: Interrupt output is disabled"]
    Cpie0 = 0,
    #[doc = "1: Interrupt output is enabled"]
    Cpie1 = 1,
}
impl From<Cpie> for bool {
    #[inline(always)]
    fn from(variant: Cpie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPIE` reader - Comparator interrupt output enable bit"]
pub type CpieR = crate::BitReader<Cpie>;
impl CpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpie {
        match self.bits {
            false => Cpie::Cpie0,
            true => Cpie::Cpie1,
        }
    }
    #[doc = "Interrupt output is disabled"]
    #[inline(always)]
    pub fn is_cpie_0(&self) -> bool {
        *self == Cpie::Cpie0
    }
    #[doc = "Interrupt output is enabled"]
    #[inline(always)]
    pub fn is_cpie_1(&self) -> bool {
        *self == Cpie::Cpie1
    }
}
#[doc = "Field `CPIE` writer - Comparator interrupt output enable bit"]
pub type CpieW<'a, REG> = crate::BitWriter<'a, REG, Cpie>;
impl<'a, REG> CpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt output is disabled"]
    #[inline(always)]
    pub fn cpie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpie::Cpie0)
    }
    #[doc = "Interrupt output is enabled"]
    #[inline(always)]
    pub fn cpie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpie::Cpie1)
    }
}
#[doc = "Comparator inverted interrupt output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpiie {
    #[doc = "0: Interrupt inverted output is disabled"]
    Cpiie0 = 0,
    #[doc = "1: Interrupt inverted output is enabled"]
    Cpiie1 = 1,
}
impl From<Cpiie> for bool {
    #[inline(always)]
    fn from(variant: Cpiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPIIE` reader - Comparator inverted interrupt output enable bit"]
pub type CpiieR = crate::BitReader<Cpiie>;
impl CpiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpiie {
        match self.bits {
            false => Cpiie::Cpiie0,
            true => Cpiie::Cpiie1,
        }
    }
    #[doc = "Interrupt inverted output is disabled"]
    #[inline(always)]
    pub fn is_cpiie_0(&self) -> bool {
        *self == Cpiie::Cpiie0
    }
    #[doc = "Interrupt inverted output is enabled"]
    #[inline(always)]
    pub fn is_cpiie_1(&self) -> bool {
        *self == Cpiie::Cpiie1
    }
}
#[doc = "Field `CPIIE` writer - Comparator inverted interrupt output enable bit"]
pub type CpiieW<'a, REG> = crate::BitWriter<'a, REG, Cpiie>;
impl<'a, REG> CpiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inverted output is disabled"]
    #[inline(always)]
    pub fn cpiie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpiie::Cpiie0)
    }
    #[doc = "Interrupt inverted output is enabled"]
    #[inline(always)]
    pub fn cpiie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpiie::Cpiie1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output value"]
    #[inline(always)]
    pub fn cpout(&self) -> CpoutR {
        CpoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn cpinv(&self) -> CpinvR {
        CpinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn cpies(&self) -> CpiesR {
        CpiesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
    #[inline(always)]
    pub fn cpflt(&self) -> CpfltR {
        CpfltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
    #[inline(always)]
    pub fn cpfltdly(&self) -> CpfltdlyR {
        CpfltdlyR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Power mode selection."]
    #[inline(always)]
    pub fn cpmsel(&self) -> CpmselR {
        CpmselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
    #[inline(always)]
    pub fn cpen(&self) -> CpenR {
        CpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
    #[inline(always)]
    pub fn cphsel(&self) -> CphselR {
        CphselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator interrupt output enable bit"]
    #[inline(always)]
    pub fn cpie(&self) -> CpieR {
        CpieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator inverted interrupt output enable bit"]
    #[inline(always)]
    pub fn cpiie(&self) -> CpiieR {
        CpiieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn cpinv(&mut self) -> CpinvW<'_, Cpctl1Spec> {
        CpinvW::new(self, 1)
    }
    #[doc = "Bit 4 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn cpies(&mut self) -> CpiesW<'_, Cpctl1Spec> {
        CpiesW::new(self, 4)
    }
    #[doc = "Bit 5 - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
    #[inline(always)]
    pub fn cpflt(&mut self) -> CpfltW<'_, Cpctl1Spec> {
        CpfltW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
    #[inline(always)]
    pub fn cpfltdly(&mut self) -> CpfltdlyW<'_, Cpctl1Spec> {
        CpfltdlyW::new(self, 6)
    }
    #[doc = "Bit 8 - Power mode selection."]
    #[inline(always)]
    pub fn cpmsel(&mut self) -> CpmselW<'_, Cpctl1Spec> {
        CpmselW::new(self, 8)
    }
    #[doc = "Bit 9 - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
    #[inline(always)]
    pub fn cpen(&mut self) -> CpenW<'_, Cpctl1Spec> {
        CpenW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
    #[inline(always)]
    pub fn cphsel(&mut self) -> CphselW<'_, Cpctl1Spec> {
        CphselW::new(self, 10)
    }
    #[doc = "Bit 14 - Comparator interrupt output enable bit"]
    #[inline(always)]
    pub fn cpie(&mut self) -> CpieW<'_, Cpctl1Spec> {
        CpieW::new(self, 14)
    }
    #[doc = "Bit 15 - Comparator inverted interrupt output enable bit"]
    #[inline(always)]
    pub fn cpiie(&mut self) -> CpiieW<'_, Cpctl1Spec> {
        CpiieW::new(self, 15)
    }
}
#[doc = "Comparator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpctl1Spec;
impl crate::RegisterSpec for Cpctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cpctl1::R`](R) reader structure"]
impl crate::Readable for Cpctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cpctl1::W`](W) writer structure"]
impl crate::Writable for Cpctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPCTL1 to value 0"]
impl crate::Resettable for Cpctl1Spec {}
