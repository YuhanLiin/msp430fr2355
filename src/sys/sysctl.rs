#[doc = "Register `SYSCTL` reader"]
pub type R = crate::R<SysctlSpec>;
#[doc = "Register `SYSCTL` writer"]
pub type W = crate::W<SysctlSpec>;
#[doc = "RAM-based interrupt vectors\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrivect {
    #[doc = "0: Interrupt vectors generated with end address TOP of lower 64KB of FRAM FFFFh"]
    Fram = 0,
    #[doc = "1: Interrupt vectors generated with end address TOP of RAM"]
    Ram = 1,
}
impl From<Sysrivect> for bool {
    #[inline(always)]
    fn from(variant: Sysrivect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRIVECT` reader - RAM-based interrupt vectors"]
pub type SysrivectR = crate::BitReader<Sysrivect>;
impl SysrivectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrivect {
        match self.bits {
            false => Sysrivect::Fram,
            true => Sysrivect::Ram,
        }
    }
    #[doc = "Interrupt vectors generated with end address TOP of lower 64KB of FRAM FFFFh"]
    #[inline(always)]
    pub fn is_fram(&self) -> bool {
        *self == Sysrivect::Fram
    }
    #[doc = "Interrupt vectors generated with end address TOP of RAM"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == Sysrivect::Ram
    }
}
#[doc = "Field `SYSRIVECT` writer - RAM-based interrupt vectors"]
pub type SysrivectW<'a, REG> = crate::BitWriter<'a, REG, Sysrivect>;
impl<'a, REG> SysrivectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt vectors generated with end address TOP of lower 64KB of FRAM FFFFh"]
    #[inline(always)]
    pub fn fram(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrivect::Fram)
    }
    #[doc = "Interrupt vectors generated with end address TOP of RAM"]
    #[inline(always)]
    pub fn ram(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrivect::Ram)
    }
}
#[doc = "PMM access protect\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspmmpe {
    #[doc = "0: Access from anywhere in memory"]
    Dis = 0,
    #[doc = "1: Access only from the protected BSL segments"]
    En = 1,
}
impl From<Syspmmpe> for bool {
    #[inline(always)]
    fn from(variant: Syspmmpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPMMPE` reader - PMM access protect"]
pub type SyspmmpeR = crate::BitReader<Syspmmpe>;
impl SyspmmpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspmmpe {
        match self.bits {
            false => Syspmmpe::Dis,
            true => Syspmmpe::En,
        }
    }
    #[doc = "Access from anywhere in memory"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Syspmmpe::Dis
    }
    #[doc = "Access only from the protected BSL segments"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Syspmmpe::En
    }
}
#[doc = "Field `SYSPMMPE` writer - PMM access protect"]
pub type SyspmmpeW<'a, REG> = crate::BitWriter<'a, REG, Syspmmpe>;
impl<'a, REG> SyspmmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access from anywhere in memory"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Syspmmpe::Dis)
    }
    #[doc = "Access only from the protected BSL segments"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Syspmmpe::En)
    }
}
#[doc = "BSL entry indication\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysbslind {
    #[doc = "0: No BSL entry sequence detected"]
    Clr = 0,
    #[doc = "1: BSL entry sequence detected"]
    Set = 1,
}
impl From<Sysbslind> for bool {
    #[inline(always)]
    fn from(variant: Sysbslind) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBSLIND` reader - BSL entry indication"]
pub type SysbslindR = crate::BitReader<Sysbslind>;
impl SysbslindR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysbslind {
        match self.bits {
            false => Sysbslind::Clr,
            true => Sysbslind::Set,
        }
    }
    #[doc = "No BSL entry sequence detected"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Sysbslind::Clr
    }
    #[doc = "BSL entry sequence detected"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Sysbslind::Set
    }
}
#[doc = "Field `SYSBSLIND` writer - BSL entry indication"]
pub type SysbslindW<'a, REG> = crate::BitWriter<'a, REG, Sysbslind>;
impl<'a, REG> SysbslindW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No BSL entry sequence detected"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbslind::Clr)
    }
    #[doc = "BSL entry sequence detected"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbslind::Set)
    }
}
#[doc = "Dedicated JTAG pins enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysjtagpin {
    #[doc = "0: Shared JTAG pins (JTAG mode selectable by JTAG/SBW sequence)"]
    Shared = 0,
    #[doc = "1: Dedicated JTAG pins (explicit 4-wire JTAG mode selection)"]
    Dedicated = 1,
}
impl From<Sysjtagpin> for bool {
    #[inline(always)]
    fn from(variant: Sysjtagpin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSJTAGPIN` reader - Dedicated JTAG pins enable"]
pub type SysjtagpinR = crate::BitReader<Sysjtagpin>;
impl SysjtagpinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysjtagpin {
        match self.bits {
            false => Sysjtagpin::Shared,
            true => Sysjtagpin::Dedicated,
        }
    }
    #[doc = "Shared JTAG pins (JTAG mode selectable by JTAG/SBW sequence)"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == Sysjtagpin::Shared
    }
    #[doc = "Dedicated JTAG pins (explicit 4-wire JTAG mode selection)"]
    #[inline(always)]
    pub fn is_dedicated(&self) -> bool {
        *self == Sysjtagpin::Dedicated
    }
}
#[doc = "Field `SYSJTAGPIN` writer - Dedicated JTAG pins enable"]
pub type SysjtagpinW<'a, REG> = crate::BitWriter<'a, REG, Sysjtagpin>;
impl<'a, REG> SysjtagpinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shared JTAG pins (JTAG mode selectable by JTAG/SBW sequence)"]
    #[inline(always)]
    pub fn shared(self) -> &'a mut crate::W<REG> {
        self.variant(Sysjtagpin::Shared)
    }
    #[doc = "Dedicated JTAG pins (explicit 4-wire JTAG mode selection)"]
    #[inline(always)]
    pub fn dedicated(self) -> &'a mut crate::W<REG> {
        self.variant(Sysjtagpin::Dedicated)
    }
}
impl R {
    #[doc = "Bit 0 - RAM-based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SysrivectR {
        SysrivectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SyspmmpeR {
        SyspmmpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - BSL entry indication"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SysbslindR {
        SysbslindR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dedicated JTAG pins enable"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SysjtagpinR {
        SysjtagpinR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM-based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&mut self) -> SysrivectW<'_, SysctlSpec> {
        SysrivectW::new(self, 0)
    }
    #[doc = "Bit 2 - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&mut self) -> SyspmmpeW<'_, SysctlSpec> {
        SyspmmpeW::new(self, 2)
    }
    #[doc = "Bit 4 - BSL entry indication"]
    #[inline(always)]
    pub fn sysbslind(&mut self) -> SysbslindW<'_, SysctlSpec> {
        SysbslindW::new(self, 4)
    }
    #[doc = "Bit 5 - Dedicated JTAG pins enable"]
    #[inline(always)]
    pub fn sysjtagpin(&mut self) -> SysjtagpinW<'_, SysctlSpec> {
        SysjtagpinW::new(self, 5)
    }
}
#[doc = "System Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSpec;
impl crate::RegisterSpec for SysctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysctl::R`](R) reader structure"]
impl crate::Readable for SysctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl::W`](W) writer structure"]
impl crate::Writable for SysctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL to value 0"]
impl crate::Resettable for SysctlSpec {}
