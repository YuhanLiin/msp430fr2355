#[doc = "Register `UCA0ABCTL` reader"]
pub type R = crate::R<Uca0abctlSpec>;
#[doc = "Register `UCA0ABCTL` writer"]
pub type W = crate::W<Uca0abctlSpec>;
#[doc = "Automatic baud-rate detect enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucabden {
    #[doc = "0: Baud-rate detection disabled. Length of break and synch field is not measured."]
    Ucabden0 = 0,
    #[doc = "1: Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly."]
    Ucabden1 = 1,
}
impl From<Ucabden> for bool {
    #[inline(always)]
    fn from(variant: Ucabden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCABDEN` reader - Automatic baud-rate detect enable"]
pub type UcabdenR = crate::BitReader<Ucabden>;
impl UcabdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucabden {
        match self.bits {
            false => Ucabden::Ucabden0,
            true => Ucabden::Ucabden1,
        }
    }
    #[doc = "Baud-rate detection disabled. Length of break and synch field is not measured."]
    #[inline(always)]
    pub fn is_ucabden_0(&self) -> bool {
        *self == Ucabden::Ucabden0
    }
    #[doc = "Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly."]
    #[inline(always)]
    pub fn is_ucabden_1(&self) -> bool {
        *self == Ucabden::Ucabden1
    }
}
#[doc = "Field `UCABDEN` writer - Automatic baud-rate detect enable"]
pub type UcabdenW<'a, REG> = crate::BitWriter<'a, REG, Ucabden>;
impl<'a, REG> UcabdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Baud-rate detection disabled. Length of break and synch field is not measured."]
    #[inline(always)]
    pub fn ucabden_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucabden::Ucabden0)
    }
    #[doc = "Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly."]
    #[inline(always)]
    pub fn ucabden_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucabden::Ucabden1)
    }
}
#[doc = "Break time out error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbtoe {
    #[doc = "0: No error"]
    Ucbtoe0 = 0,
    #[doc = "1: Length of break field exceeded 22 bit times"]
    Ucbtoe1 = 1,
}
impl From<Ucbtoe> for bool {
    #[inline(always)]
    fn from(variant: Ucbtoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBTOE` reader - Break time out error"]
pub type UcbtoeR = crate::BitReader<Ucbtoe>;
impl UcbtoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbtoe {
        match self.bits {
            false => Ucbtoe::Ucbtoe0,
            true => Ucbtoe::Ucbtoe1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ucbtoe_0(&self) -> bool {
        *self == Ucbtoe::Ucbtoe0
    }
    #[doc = "Length of break field exceeded 22 bit times"]
    #[inline(always)]
    pub fn is_ucbtoe_1(&self) -> bool {
        *self == Ucbtoe::Ucbtoe1
    }
}
#[doc = "Field `UCBTOE` writer - Break time out error"]
pub type UcbtoeW<'a, REG> = crate::BitWriter<'a, REG, Ucbtoe>;
impl<'a, REG> UcbtoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucbtoe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbtoe::Ucbtoe0)
    }
    #[doc = "Length of break field exceeded 22 bit times"]
    #[inline(always)]
    pub fn ucbtoe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbtoe::Ucbtoe1)
    }
}
#[doc = "Synch field time out error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucstoe {
    #[doc = "0: No error"]
    Ucstoe0 = 0,
    #[doc = "1: Length of synch field exceeded measurable time"]
    Ucstoe1 = 1,
}
impl From<Ucstoe> for bool {
    #[inline(always)]
    fn from(variant: Ucstoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTOE` reader - Synch field time out error"]
pub type UcstoeR = crate::BitReader<Ucstoe>;
impl UcstoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucstoe {
        match self.bits {
            false => Ucstoe::Ucstoe0,
            true => Ucstoe::Ucstoe1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ucstoe_0(&self) -> bool {
        *self == Ucstoe::Ucstoe0
    }
    #[doc = "Length of synch field exceeded measurable time"]
    #[inline(always)]
    pub fn is_ucstoe_1(&self) -> bool {
        *self == Ucstoe::Ucstoe1
    }
}
#[doc = "Field `UCSTOE` writer - Synch field time out error"]
pub type UcstoeW<'a, REG> = crate::BitWriter<'a, REG, Ucstoe>;
impl<'a, REG> UcstoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucstoe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstoe::Ucstoe0)
    }
    #[doc = "Length of synch field exceeded measurable time"]
    #[inline(always)]
    pub fn ucstoe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucstoe::Ucstoe1)
    }
}
#[doc = "Break/synch delimiter length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucdelim {
    #[doc = "0: 1 bit time"]
    Ucdelim0 = 0,
    #[doc = "1: 2 bit times"]
    Ucdelim1 = 1,
    #[doc = "2: 3 bit times"]
    Ucdelim2 = 2,
    #[doc = "3: 4 bit times"]
    Ucdelim3 = 3,
}
impl From<Ucdelim> for u8 {
    #[inline(always)]
    fn from(variant: Ucdelim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucdelim {
    type Ux = u8;
}
impl crate::IsEnum for Ucdelim {}
#[doc = "Field `UCDELIM` reader - Break/synch delimiter length"]
pub type UcdelimR = crate::FieldReader<Ucdelim>;
impl UcdelimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucdelim {
        match self.bits {
            0 => Ucdelim::Ucdelim0,
            1 => Ucdelim::Ucdelim1,
            2 => Ucdelim::Ucdelim2,
            3 => Ucdelim::Ucdelim3,
            _ => unreachable!(),
        }
    }
    #[doc = "1 bit time"]
    #[inline(always)]
    pub fn is_ucdelim_0(&self) -> bool {
        *self == Ucdelim::Ucdelim0
    }
    #[doc = "2 bit times"]
    #[inline(always)]
    pub fn is_ucdelim_1(&self) -> bool {
        *self == Ucdelim::Ucdelim1
    }
    #[doc = "3 bit times"]
    #[inline(always)]
    pub fn is_ucdelim_2(&self) -> bool {
        *self == Ucdelim::Ucdelim2
    }
    #[doc = "4 bit times"]
    #[inline(always)]
    pub fn is_ucdelim_3(&self) -> bool {
        *self == Ucdelim::Ucdelim3
    }
}
#[doc = "Field `UCDELIM` writer - Break/synch delimiter length"]
pub type UcdelimW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucdelim, crate::Safe>;
impl<'a, REG> UcdelimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 bit time"]
    #[inline(always)]
    pub fn ucdelim_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucdelim::Ucdelim0)
    }
    #[doc = "2 bit times"]
    #[inline(always)]
    pub fn ucdelim_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucdelim::Ucdelim1)
    }
    #[doc = "3 bit times"]
    #[inline(always)]
    pub fn ucdelim_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucdelim::Ucdelim2)
    }
    #[doc = "4 bit times"]
    #[inline(always)]
    pub fn ucdelim_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucdelim::Ucdelim3)
    }
}
impl R {
    #[doc = "Bit 0 - Automatic baud-rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UcabdenR {
        UcabdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Break time out error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UcbtoeR {
        UcbtoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synch field time out error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UcstoeR {
        UcstoeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Break/synch delimiter length"]
    #[inline(always)]
    pub fn ucdelim(&self) -> UcdelimR {
        UcdelimR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic baud-rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UcabdenW<'_, Uca0abctlSpec> {
        UcabdenW::new(self, 0)
    }
    #[doc = "Bit 2 - Break time out error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UcbtoeW<'_, Uca0abctlSpec> {
        UcbtoeW::new(self, 2)
    }
    #[doc = "Bit 3 - Synch field time out error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UcstoeW<'_, Uca0abctlSpec> {
        UcstoeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Break/synch delimiter length"]
    #[inline(always)]
    pub fn ucdelim(&mut self) -> UcdelimW<'_, Uca0abctlSpec> {
        UcdelimW::new(self, 4)
    }
}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0abctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0abctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0abctlSpec;
impl crate::RegisterSpec for Uca0abctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0abctl::R`](R) reader structure"]
impl crate::Readable for Uca0abctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0abctl::W`](W) writer structure"]
impl crate::Writable for Uca0abctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0ABCTL to value 0"]
impl crate::Resettable for Uca0abctlSpec {}
