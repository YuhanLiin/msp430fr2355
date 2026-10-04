#[doc = "Register `CP1INT` reader"]
pub type R = crate::R<Cp1intSpec>;
#[doc = "Register `CP1INT` writer"]
pub type W = crate::W<Cp1intSpec>;
#[doc = "Comparator output interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpifg {
    #[doc = "0: No interrupt pending."]
    Cpifg0 = 0,
    #[doc = "1: Output interrupt pending."]
    Cpifg1 = 1,
}
impl From<Cpifg> for bool {
    #[inline(always)]
    fn from(variant: Cpifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPIFG` reader - Comparator output interrupt flag"]
pub type CpifgR = crate::BitReader<Cpifg>;
impl CpifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpifg {
        match self.bits {
            false => Cpifg::Cpifg0,
            true => Cpifg::Cpifg1,
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn is_cpifg_0(&self) -> bool {
        *self == Cpifg::Cpifg0
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn is_cpifg_1(&self) -> bool {
        *self == Cpifg::Cpifg1
    }
}
#[doc = "Field `CPIFG` writer - Comparator output interrupt flag"]
pub type CpifgW<'a, REG> = crate::BitWriter<'a, REG, Cpifg>;
impl<'a, REG> CpifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn cpifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpifg::Cpifg0)
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn cpifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpifg::Cpifg1)
    }
}
#[doc = "Comparator output inverted interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpiifg {
    #[doc = "0: No interrupt pending."]
    Cpiifg0 = 0,
    #[doc = "1: Output interrupt pending."]
    Cpiifg1 = 1,
}
impl From<Cpiifg> for bool {
    #[inline(always)]
    fn from(variant: Cpiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPIIFG` reader - Comparator output inverted interrupt flag"]
pub type CpiifgR = crate::BitReader<Cpiifg>;
impl CpiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpiifg {
        match self.bits {
            false => Cpiifg::Cpiifg0,
            true => Cpiifg::Cpiifg1,
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn is_cpiifg_0(&self) -> bool {
        *self == Cpiifg::Cpiifg0
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn is_cpiifg_1(&self) -> bool {
        *self == Cpiifg::Cpiifg1
    }
}
#[doc = "Field `CPIIFG` writer - Comparator output inverted interrupt flag"]
pub type CpiifgW<'a, REG> = crate::BitWriter<'a, REG, Cpiifg>;
impl<'a, REG> CpiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn cpiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpiifg::Cpiifg0)
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn cpiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpiifg::Cpiifg1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn cpifg(&self) -> CpifgR {
        CpifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn cpiifg(&self) -> CpiifgR {
        CpiifgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn cpifg(&mut self) -> CpifgW<'_, Cp1intSpec> {
        CpifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn cpiifg(&mut self) -> CpiifgW<'_, Cp1intSpec> {
        CpiifgW::new(self, 1)
    }
}
#[doc = "Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp1int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp1int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cp1intSpec;
impl crate::RegisterSpec for Cp1intSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cp1int::R`](R) reader structure"]
impl crate::Readable for Cp1intSpec {}
#[doc = "`write(|w| ..)` method takes [`cp1int::W`](W) writer structure"]
impl crate::Writable for Cp1intSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CP1INT to value 0"]
impl crate::Resettable for Cp1intSpec {}
