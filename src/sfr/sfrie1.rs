#[doc = "Register `SFRIE1` reader"]
pub type R = crate::R<Sfrie1Spec>;
#[doc = "Register `SFRIE1` writer"]
pub type W = crate::W<Sfrie1Spec>;
#[doc = "Watchdog timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtie {
    #[doc = "0: Interrupts disabled"]
    Disable = 0,
    #[doc = "1: Interrupts enabled"]
    Enable = 1,
}
impl From<Wdtie> for bool {
    #[inline(always)]
    fn from(variant: Wdtie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTIE` reader - Watchdog timer interrupt enable"]
pub type WdtieR = crate::BitReader<Wdtie>;
impl WdtieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtie {
        match self.bits {
            false => Wdtie::Disable,
            true => Wdtie::Enable,
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wdtie::Disable
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wdtie::Enable
    }
}
#[doc = "Field `WDTIE` writer - Watchdog timer interrupt enable"]
pub type WdtieW<'a, REG> = crate::BitWriter<'a, REG, Wdtie>;
impl<'a, REG> WdtieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtie::Disable)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtie::Enable)
    }
}
#[doc = "Oscillator fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ofie {
    #[doc = "0: Interrupts disabled"]
    Disable = 0,
    #[doc = "1: Interrupts enabled"]
    Enable = 1,
}
impl From<Ofie> for bool {
    #[inline(always)]
    fn from(variant: Ofie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFIE` reader - Oscillator fault interrupt enable"]
pub type OfieR = crate::BitReader<Ofie>;
impl OfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ofie {
        match self.bits {
            false => Ofie::Disable,
            true => Ofie::Enable,
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ofie::Disable
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ofie::Enable
    }
}
#[doc = "Field `OFIE` writer - Oscillator fault interrupt enable"]
pub type OfieW<'a, REG> = crate::BitWriter<'a, REG, Ofie>;
impl<'a, REG> OfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ofie::Disable)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ofie::Enable)
    }
}
#[doc = "Vacant memory access interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vmaie {
    #[doc = "0: Interrupts disabled"]
    Disable = 0,
    #[doc = "1: Interrupts enabled"]
    Enable = 1,
}
impl From<Vmaie> for bool {
    #[inline(always)]
    fn from(variant: Vmaie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMAIE` reader - Vacant memory access interrupt enable"]
pub type VmaieR = crate::BitReader<Vmaie>;
impl VmaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vmaie {
        match self.bits {
            false => Vmaie::Disable,
            true => Vmaie::Enable,
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Vmaie::Disable
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Vmaie::Enable
    }
}
#[doc = "Field `VMAIE` writer - Vacant memory access interrupt enable"]
pub type VmaieW<'a, REG> = crate::BitWriter<'a, REG, Vmaie>;
impl<'a, REG> VmaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Vmaie::Disable)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Vmaie::Enable)
    }
}
#[doc = "NMI pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmiie {
    #[doc = "0: Interrupts disabled"]
    Disable = 0,
    #[doc = "1: Interrupts enabled"]
    Enable = 1,
}
impl From<Nmiie> for bool {
    #[inline(always)]
    fn from(variant: Nmiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIIE` reader - NMI pin interrupt enable"]
pub type NmiieR = crate::BitReader<Nmiie>;
impl NmiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmiie {
        match self.bits {
            false => Nmiie::Disable,
            true => Nmiie::Enable,
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Nmiie::Disable
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Nmiie::Enable
    }
}
#[doc = "Field `NMIIE` writer - NMI pin interrupt enable"]
pub type NmiieW<'a, REG> = crate::BitWriter<'a, REG, Nmiie>;
impl<'a, REG> NmiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiie::Disable)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiie::Enable)
    }
}
#[doc = "JTAG mailbox input interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbinie {
    #[doc = "0: Interrupts disabled"]
    Disable = 0,
    #[doc = "1: Interrupts enabled"]
    Enable = 1,
}
impl From<Jmbinie> for bool {
    #[inline(always)]
    fn from(variant: Jmbinie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBINIE` reader - JTAG mailbox input interrupt enable"]
pub type JmbinieR = crate::BitReader<Jmbinie>;
impl JmbinieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbinie {
        match self.bits {
            false => Jmbinie::Disable,
            true => Jmbinie::Enable,
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Jmbinie::Disable
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Jmbinie::Enable
    }
}
#[doc = "Field `JMBINIE` writer - JTAG mailbox input interrupt enable"]
pub type JmbinieW<'a, REG> = crate::BitWriter<'a, REG, Jmbinie>;
impl<'a, REG> JmbinieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbinie::Disable)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbinie::Enable)
    }
}
#[doc = "JTAG mailbox output interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmboutie {
    #[doc = "0: Interrupts disabled"]
    Disable = 0,
    #[doc = "1: Interrupts enabled"]
    Enable = 1,
}
impl From<Jmboutie> for bool {
    #[inline(always)]
    fn from(variant: Jmboutie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBOUTIE` reader - JTAG mailbox output interrupt enable"]
pub type JmboutieR = crate::BitReader<Jmboutie>;
impl JmboutieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmboutie {
        match self.bits {
            false => Jmboutie::Disable,
            true => Jmboutie::Enable,
        }
    }
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Jmboutie::Disable
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Jmboutie::Enable
    }
}
#[doc = "Field `JMBOUTIE` writer - JTAG mailbox output interrupt enable"]
pub type JmboutieW<'a, REG> = crate::BitWriter<'a, REG, Jmboutie>;
impl<'a, REG> JmboutieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Jmboutie::Disable)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Jmboutie::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog timer interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WdtieR {
        WdtieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OfieR {
        OfieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VmaieR {
        VmaieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI pin interrupt enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NmiieR {
        NmiieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JmbinieR {
        JmbinieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JmboutieR {
        JmboutieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog timer interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WdtieW<'_, Sfrie1Spec> {
        WdtieW::new(self, 0)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OfieW<'_, Sfrie1Spec> {
        OfieW::new(self, 1)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt enable"]
    #[inline(always)]
    pub fn vmaie(&mut self) -> VmaieW<'_, Sfrie1Spec> {
        VmaieW::new(self, 3)
    }
    #[doc = "Bit 4 - NMI pin interrupt enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NmiieW<'_, Sfrie1Spec> {
        NmiieW::new(self, 4)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt enable"]
    #[inline(always)]
    pub fn jmbinie(&mut self) -> JmbinieW<'_, Sfrie1Spec> {
        JmbinieW::new(self, 6)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt enable"]
    #[inline(always)]
    pub fn jmboutie(&mut self) -> JmboutieW<'_, Sfrie1Spec> {
        JmboutieW::new(self, 7)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrie1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrie1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sfrie1Spec;
impl crate::RegisterSpec for Sfrie1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrie1::R`](R) reader structure"]
impl crate::Readable for Sfrie1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfrie1::W`](W) writer structure"]
impl crate::Writable for Sfrie1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRIE1 to value 0"]
impl crate::Resettable for Sfrie1Spec {}
