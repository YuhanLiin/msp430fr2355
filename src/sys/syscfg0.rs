#[doc = "Register `SYSCFG0` reader"]
pub type R = crate::R<Syscfg0Spec>;
#[doc = "Register `SYSCFG0` writer"]
pub type W = crate::W<Syscfg0Spec>;
#[doc = "Program FRAM write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfwp {
    #[doc = "0: Program FRAM write enable"]
    Pfwp0 = 0,
    #[doc = "1: Program FRAM write protected (not writable)"]
    Pfwp1 = 1,
}
impl From<Pfwp> for bool {
    #[inline(always)]
    fn from(variant: Pfwp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFWP` reader - Program FRAM write protection"]
pub type PfwpR = crate::BitReader<Pfwp>;
impl PfwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfwp {
        match self.bits {
            false => Pfwp::Pfwp0,
            true => Pfwp::Pfwp1,
        }
    }
    #[doc = "Program FRAM write enable"]
    #[inline(always)]
    pub fn is_pfwp_0(&self) -> bool {
        *self == Pfwp::Pfwp0
    }
    #[doc = "Program FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn is_pfwp_1(&self) -> bool {
        *self == Pfwp::Pfwp1
    }
}
#[doc = "Field `PFWP` writer - Program FRAM write protection"]
pub type PfwpW<'a, REG> = crate::BitWriter<'a, REG, Pfwp>;
impl<'a, REG> PfwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Program FRAM write enable"]
    #[inline(always)]
    pub fn pfwp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pfwp::Pfwp0)
    }
    #[doc = "Program FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn pfwp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfwp::Pfwp1)
    }
}
#[doc = "Data FRAM write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfwp {
    #[doc = "0: Data FRAM write enable"]
    Dfwp0 = 0,
    #[doc = "1: Data FRAM write protected (not writable)"]
    Dfwp1 = 1,
}
impl From<Dfwp> for bool {
    #[inline(always)]
    fn from(variant: Dfwp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFWP` reader - Data FRAM write protection"]
pub type DfwpR = crate::BitReader<Dfwp>;
impl DfwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfwp {
        match self.bits {
            false => Dfwp::Dfwp0,
            true => Dfwp::Dfwp1,
        }
    }
    #[doc = "Data FRAM write enable"]
    #[inline(always)]
    pub fn is_dfwp_0(&self) -> bool {
        *self == Dfwp::Dfwp0
    }
    #[doc = "Data FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn is_dfwp_1(&self) -> bool {
        *self == Dfwp::Dfwp1
    }
}
#[doc = "Field `DFWP` writer - Data FRAM write protection"]
pub type DfwpW<'a, REG> = crate::BitWriter<'a, REG, Dfwp>;
impl<'a, REG> DfwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data FRAM write enable"]
    #[inline(always)]
    pub fn dfwp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfwp::Dfwp0)
    }
    #[doc = "Data FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn dfwp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfwp::Dfwp1)
    }
}
#[doc = "Field `FRWPOA` reader - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
pub type FrwpoaR = crate::FieldReader;
#[doc = "Field `FRWPOA` writer - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
pub type FrwpoaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FRWPPW` reader - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
pub type FrwppwR = crate::FieldReader;
#[doc = "Field `FRWPPW` writer - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
pub type FrwppwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Program FRAM write protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PfwpR {
        PfwpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FRAM write protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DfwpR {
        DfwpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
    #[inline(always)]
    pub fn frwpoa(&self) -> FrwpoaR {
        FrwpoaR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
    #[inline(always)]
    pub fn frwppw(&self) -> FrwppwR {
        FrwppwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM write protection"]
    #[inline(always)]
    pub fn pfwp(&mut self) -> PfwpW<'_, Syscfg0Spec> {
        PfwpW::new(self, 0)
    }
    #[doc = "Bit 1 - Data FRAM write protection"]
    #[inline(always)]
    pub fn dfwp(&mut self) -> DfwpW<'_, Syscfg0Spec> {
        DfwpW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
    #[inline(always)]
    pub fn frwpoa(&mut self) -> FrwpoaW<'_, Syscfg0Spec> {
        FrwpoaW::new(self, 2)
    }
    #[doc = "Bits 8:15 - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
    #[inline(always)]
    pub fn frwppw(&mut self) -> FrwppwW<'_, Syscfg0Spec> {
        FrwppwW::new(self, 8)
    }
}
#[doc = "System Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg0Spec;
impl crate::RegisterSpec for Syscfg0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg0::R`](R) reader structure"]
impl crate::Readable for Syscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg0::W`](W) writer structure"]
impl crate::Writable for Syscfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for Syscfg0Spec {}
