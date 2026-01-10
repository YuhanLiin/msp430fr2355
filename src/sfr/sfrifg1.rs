#[doc = "Register `SFRIFG1` reader"]
pub type R = crate::R<Sfrifg1Spec>;
#[doc = "Register `SFRIFG1` writer"]
pub type W = crate::W<Sfrifg1Spec>;
#[doc = "Watchdog timer interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtifg {
    #[doc = "0: No interrupt pending"]
    Wdtifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Wdtifg1 = 1,
}
impl From<Wdtifg> for bool {
    #[inline(always)]
    fn from(variant: Wdtifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTIFG` reader - Watchdog timer interrupt flag"]
pub type WdtifgR = crate::BitReader<Wdtifg>;
impl WdtifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtifg {
        match self.bits {
            false => Wdtifg::Wdtifg0,
            true => Wdtifg::Wdtifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_wdtifg_0(&self) -> bool {
        *self == Wdtifg::Wdtifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_wdtifg_1(&self) -> bool {
        *self == Wdtifg::Wdtifg1
    }
}
#[doc = "Field `WDTIFG` writer - Watchdog timer interrupt flag"]
pub type WdtifgW<'a, REG> = crate::BitWriter<'a, REG, Wdtifg>;
impl<'a, REG> WdtifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn wdtifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtifg::Wdtifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn wdtifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtifg::Wdtifg1)
    }
}
#[doc = "Oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ofifg {
    #[doc = "0: No interrupt pending"]
    Ofifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ofifg1 = 1,
}
impl From<Ofifg> for bool {
    #[inline(always)]
    fn from(variant: Ofifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFIFG` reader - Oscillator fault interrupt flag"]
pub type OfifgR = crate::BitReader<Ofifg>;
impl OfifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ofifg {
        match self.bits {
            false => Ofifg::Ofifg0,
            true => Ofifg::Ofifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ofifg_0(&self) -> bool {
        *self == Ofifg::Ofifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ofifg_1(&self) -> bool {
        *self == Ofifg::Ofifg1
    }
}
#[doc = "Field `OFIFG` writer - Oscillator fault interrupt flag"]
pub type OfifgW<'a, REG> = crate::BitWriter<'a, REG, Ofifg>;
impl<'a, REG> OfifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ofifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ofifg::Ofifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ofifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ofifg::Ofifg1)
    }
}
#[doc = "Vacant memory access interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vmaifg {
    #[doc = "0: No interrupt pending"]
    Vmaifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Vmaifg1 = 1,
}
impl From<Vmaifg> for bool {
    #[inline(always)]
    fn from(variant: Vmaifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMAIFG` reader - Vacant memory access interrupt flag"]
pub type VmaifgR = crate::BitReader<Vmaifg>;
impl VmaifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vmaifg {
        match self.bits {
            false => Vmaifg::Vmaifg0,
            true => Vmaifg::Vmaifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_vmaifg_0(&self) -> bool {
        *self == Vmaifg::Vmaifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_vmaifg_1(&self) -> bool {
        *self == Vmaifg::Vmaifg1
    }
}
#[doc = "Field `VMAIFG` writer - Vacant memory access interrupt flag"]
pub type VmaifgW<'a, REG> = crate::BitWriter<'a, REG, Vmaifg>;
impl<'a, REG> VmaifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn vmaifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vmaifg::Vmaifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn vmaifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vmaifg::Vmaifg1)
    }
}
#[doc = "NMI pin interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmiifg {
    #[doc = "0: No interrupt pending"]
    Nmiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Nmiifg1 = 1,
}
impl From<Nmiifg> for bool {
    #[inline(always)]
    fn from(variant: Nmiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIIFG` reader - NMI pin interrupt flag"]
pub type NmiifgR = crate::BitReader<Nmiifg>;
impl NmiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmiifg {
        match self.bits {
            false => Nmiifg::Nmiifg0,
            true => Nmiifg::Nmiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_nmiifg_0(&self) -> bool {
        *self == Nmiifg::Nmiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_nmiifg_1(&self) -> bool {
        *self == Nmiifg::Nmiifg1
    }
}
#[doc = "Field `NMIIFG` writer - NMI pin interrupt flag"]
pub type NmiifgW<'a, REG> = crate::BitWriter<'a, REG, Nmiifg>;
impl<'a, REG> NmiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn nmiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiifg::Nmiifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn nmiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiifg::Nmiifg1)
    }
}
#[doc = "JTAG mailbox input interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbinifg {
    #[doc = "0: No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read"]
    Jmbinifg0 = 0,
    #[doc = "1: Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module."]
    Jmbinifg1 = 1,
}
impl From<Jmbinifg> for bool {
    #[inline(always)]
    fn from(variant: Jmbinifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBINIFG` reader - JTAG mailbox input interrupt flag"]
pub type JmbinifgR = crate::BitReader<Jmbinifg>;
impl JmbinifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbinifg {
        match self.bits {
            false => Jmbinifg::Jmbinifg0,
            true => Jmbinifg::Jmbinifg1,
        }
    }
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read"]
    #[inline(always)]
    pub fn is_jmbinifg_0(&self) -> bool {
        *self == Jmbinifg::Jmbinifg0
    }
    #[doc = "Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module."]
    #[inline(always)]
    pub fn is_jmbinifg_1(&self) -> bool {
        *self == Jmbinifg::Jmbinifg1
    }
}
#[doc = "Field `JMBINIFG` writer - JTAG mailbox input interrupt flag"]
pub type JmbinifgW<'a, REG> = crate::BitWriter<'a, REG, Jmbinifg>;
impl<'a, REG> JmbinifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read"]
    #[inline(always)]
    pub fn jmbinifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbinifg::Jmbinifg0)
    }
    #[doc = "Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module."]
    #[inline(always)]
    pub fn jmbinifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbinifg::Jmbinifg1)
    }
}
#[doc = "JTAG mailbox output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmboutifg {
    #[doc = "0: No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read."]
    Jmboutifg0 = 0,
    #[doc = "1: Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU."]
    Jmboutifg1 = 1,
}
impl From<Jmboutifg> for bool {
    #[inline(always)]
    fn from(variant: Jmboutifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBOUTIFG` reader - JTAG mailbox output interrupt flag"]
pub type JmboutifgR = crate::BitReader<Jmboutifg>;
impl JmboutifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmboutifg {
        match self.bits {
            false => Jmboutifg::Jmboutifg0,
            true => Jmboutifg::Jmboutifg1,
        }
    }
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read."]
    #[inline(always)]
    pub fn is_jmboutifg_0(&self) -> bool {
        *self == Jmboutifg::Jmboutifg0
    }
    #[doc = "Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU."]
    #[inline(always)]
    pub fn is_jmboutifg_1(&self) -> bool {
        *self == Jmboutifg::Jmboutifg1
    }
}
#[doc = "Field `JMBOUTIFG` writer - JTAG mailbox output interrupt flag"]
pub type JmboutifgW<'a, REG> = crate::BitWriter<'a, REG, Jmboutifg>;
impl<'a, REG> JmboutifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read."]
    #[inline(always)]
    pub fn jmboutifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Jmboutifg::Jmboutifg0)
    }
    #[doc = "Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU."]
    #[inline(always)]
    pub fn jmboutifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Jmboutifg::Jmboutifg1)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog timer interrupt flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WdtifgR {
        WdtifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OfifgR {
        OfifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt flag"]
    #[inline(always)]
    pub fn vmaifg(&self) -> VmaifgR {
        VmaifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI pin interrupt flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NmiifgR {
        NmiifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt flag"]
    #[inline(always)]
    pub fn jmbinifg(&self) -> JmbinifgR {
        JmbinifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt flag"]
    #[inline(always)]
    pub fn jmboutifg(&self) -> JmboutifgR {
        JmboutifgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog timer interrupt flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WdtifgW<'_, Sfrifg1Spec> {
        WdtifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OfifgW<'_, Sfrifg1Spec> {
        OfifgW::new(self, 1)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt flag"]
    #[inline(always)]
    pub fn vmaifg(&mut self) -> VmaifgW<'_, Sfrifg1Spec> {
        VmaifgW::new(self, 3)
    }
    #[doc = "Bit 4 - NMI pin interrupt flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NmiifgW<'_, Sfrifg1Spec> {
        NmiifgW::new(self, 4)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt flag"]
    #[inline(always)]
    pub fn jmbinifg(&mut self) -> JmbinifgW<'_, Sfrifg1Spec> {
        JmbinifgW::new(self, 6)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt flag"]
    #[inline(always)]
    pub fn jmboutifg(&mut self) -> JmboutifgW<'_, Sfrifg1Spec> {
        JmboutifgW::new(self, 7)
    }
}
#[doc = "Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrifg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrifg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sfrifg1Spec;
impl crate::RegisterSpec for Sfrifg1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrifg1::R`](R) reader structure"]
impl crate::Readable for Sfrifg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfrifg1::W`](W) writer structure"]
impl crate::Writable for Sfrifg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRIFG1 to value 0"]
impl crate::Resettable for Sfrifg1Spec {}
