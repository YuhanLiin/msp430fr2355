#[doc = "Register `SFRRPCR` reader"]
pub type R = crate::R<SfrrpcrSpec>;
#[doc = "Register `SFRRPCR` writer"]
pub type W = crate::W<SfrrpcrSpec>;
#[doc = "NMI select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysnmi {
    #[doc = "0: Reset function"]
    Reset = 0,
    #[doc = "1: NMI function"]
    Nmi = 1,
}
impl From<Sysnmi> for bool {
    #[inline(always)]
    fn from(variant: Sysnmi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSNMI` reader - NMI select"]
pub type SysnmiR = crate::BitReader<Sysnmi>;
impl SysnmiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysnmi {
        match self.bits {
            false => Sysnmi::Reset,
            true => Sysnmi::Nmi,
        }
    }
    #[doc = "Reset function"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Sysnmi::Reset
    }
    #[doc = "NMI function"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == Sysnmi::Nmi
    }
}
#[doc = "Field `SYSNMI` writer - NMI select"]
pub type SysnmiW<'a, REG> = crate::BitWriter<'a, REG, Sysnmi>;
impl<'a, REG> SysnmiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset function"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sysnmi::Reset)
    }
    #[doc = "NMI function"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut crate::W<REG> {
        self.variant(Sysnmi::Nmi)
    }
}
#[doc = "NMI edge select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysnmiies {
    #[doc = "0: NMI on rising edge"]
    Rising = 0,
    #[doc = "1: NMI on falling edge"]
    Falling = 1,
}
impl From<Sysnmiies> for bool {
    #[inline(always)]
    fn from(variant: Sysnmiies) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSNMIIES` reader - NMI edge select"]
pub type SysnmiiesR = crate::BitReader<Sysnmiies>;
impl SysnmiiesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysnmiies {
        match self.bits {
            false => Sysnmiies::Rising,
            true => Sysnmiies::Falling,
        }
    }
    #[doc = "NMI on rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Sysnmiies::Rising
    }
    #[doc = "NMI on falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Sysnmiies::Falling
    }
}
#[doc = "Field `SYSNMIIES` writer - NMI edge select"]
pub type SysnmiiesW<'a, REG> = crate::BitWriter<'a, REG, Sysnmiies>;
impl<'a, REG> SysnmiiesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NMI on rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Sysnmiies::Rising)
    }
    #[doc = "NMI on falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Sysnmiies::Falling)
    }
}
#[doc = "Reset resistor pin pullup or pulldown\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrstup {
    #[doc = "0: Pulldown is selected"]
    Pulldown = 0,
    #[doc = "1: Pullup is selected"]
    Pullup = 1,
}
impl From<Sysrstup> for bool {
    #[inline(always)]
    fn from(variant: Sysrstup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRSTUP` reader - Reset resistor pin pullup or pulldown"]
pub type SysrstupR = crate::BitReader<Sysrstup>;
impl SysrstupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrstup {
        match self.bits {
            false => Sysrstup::Pulldown,
            true => Sysrstup::Pullup,
        }
    }
    #[doc = "Pulldown is selected"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Sysrstup::Pulldown
    }
    #[doc = "Pullup is selected"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Sysrstup::Pullup
    }
}
#[doc = "Field `SYSRSTUP` writer - Reset resistor pin pullup or pulldown"]
pub type SysrstupW<'a, REG> = crate::BitWriter<'a, REG, Sysrstup>;
impl<'a, REG> SysrstupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pulldown is selected"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstup::Pulldown)
    }
    #[doc = "Pullup is selected"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstup::Pullup)
    }
}
#[doc = "Reset pin resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrstre {
    #[doc = "0: Pullup or pulldown resistor at the RST/NMI pin is disabled"]
    Disable = 0,
    #[doc = "1: Pullup or pulldown resistor at the RST/NMI pin is enabled"]
    Enable = 1,
}
impl From<Sysrstre> for bool {
    #[inline(always)]
    fn from(variant: Sysrstre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRSTRE` reader - Reset pin resistor enable"]
pub type SysrstreR = crate::BitReader<Sysrstre>;
impl SysrstreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrstre {
        match self.bits {
            false => Sysrstre::Disable,
            true => Sysrstre::Enable,
        }
    }
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sysrstre::Disable
    }
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sysrstre::Enable
    }
}
#[doc = "Field `SYSRSTRE` writer - Reset pin resistor enable"]
pub type SysrstreW<'a, REG> = crate::BitWriter<'a, REG, Sysrstre>;
impl<'a, REG> SysrstreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstre::Disable)
    }
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstre::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SysnmiR {
        SysnmiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SysnmiiesR {
        SysnmiiesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset resistor pin pullup or pulldown"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SysrstupR {
        SysrstupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset pin resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SysrstreR {
        SysrstreR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&mut self) -> SysnmiW<'_, SfrrpcrSpec> {
        SysnmiW::new(self, 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&mut self) -> SysnmiiesW<'_, SfrrpcrSpec> {
        SysnmiiesW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset resistor pin pullup or pulldown"]
    #[inline(always)]
    pub fn sysrstup(&mut self) -> SysrstupW<'_, SfrrpcrSpec> {
        SysrstupW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset pin resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&mut self) -> SysrstreW<'_, SfrrpcrSpec> {
        SysrstreW::new(self, 3)
    }
}
#[doc = "Reset Pin Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrrpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrrpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrrpcrSpec;
impl crate::RegisterSpec for SfrrpcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrrpcr::R`](R) reader structure"]
impl crate::Readable for SfrrpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrrpcr::W`](W) writer structure"]
impl crate::Writable for SfrrpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRRPCR to value 0"]
impl crate::Resettable for SfrrpcrSpec {}
