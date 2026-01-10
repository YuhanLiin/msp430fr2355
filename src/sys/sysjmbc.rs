#[doc = "Register `SYSJMBC` reader"]
pub type R = crate::R<SysjmbcSpec>;
#[doc = "Register `SYSJMBC` writer"]
pub type W = crate::W<SysjmbcSpec>;
#[doc = "Incoming JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbin0fg {
    #[doc = "0: JMBI0 has no new data"]
    Jmbin0fg0 = 0,
    #[doc = "1: JMBI0 has new data available"]
    Jmbin0fg1 = 1,
}
impl From<Jmbin0fg> for bool {
    #[inline(always)]
    fn from(variant: Jmbin0fg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBIN0FG` reader - Incoming JTAG Mailbox 0 flag"]
pub type Jmbin0fgR = crate::BitReader<Jmbin0fg>;
impl Jmbin0fgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbin0fg {
        match self.bits {
            false => Jmbin0fg::Jmbin0fg0,
            true => Jmbin0fg::Jmbin0fg1,
        }
    }
    #[doc = "JMBI0 has no new data"]
    #[inline(always)]
    pub fn is_jmbin0fg_0(&self) -> bool {
        *self == Jmbin0fg::Jmbin0fg0
    }
    #[doc = "JMBI0 has new data available"]
    #[inline(always)]
    pub fn is_jmbin0fg_1(&self) -> bool {
        *self == Jmbin0fg::Jmbin0fg1
    }
}
#[doc = "Field `JMBIN0FG` writer - Incoming JTAG Mailbox 0 flag"]
pub type Jmbin0fgW<'a, REG> = crate::BitWriter<'a, REG, Jmbin0fg>;
impl<'a, REG> Jmbin0fgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JMBI0 has no new data"]
    #[inline(always)]
    pub fn jmbin0fg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbin0fg::Jmbin0fg0)
    }
    #[doc = "JMBI0 has new data available"]
    #[inline(always)]
    pub fn jmbin0fg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbin0fg::Jmbin0fg1)
    }
}
#[doc = "Incoming JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbin1fg {
    #[doc = "0: JMBI1 has no new data"]
    Jmbin1fg0 = 0,
    #[doc = "1: JMBI1 has new data available"]
    Jmbin1fg1 = 1,
}
impl From<Jmbin1fg> for bool {
    #[inline(always)]
    fn from(variant: Jmbin1fg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBIN1FG` reader - Incoming JTAG Mailbox 1 flag"]
pub type Jmbin1fgR = crate::BitReader<Jmbin1fg>;
impl Jmbin1fgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbin1fg {
        match self.bits {
            false => Jmbin1fg::Jmbin1fg0,
            true => Jmbin1fg::Jmbin1fg1,
        }
    }
    #[doc = "JMBI1 has no new data"]
    #[inline(always)]
    pub fn is_jmbin1fg_0(&self) -> bool {
        *self == Jmbin1fg::Jmbin1fg0
    }
    #[doc = "JMBI1 has new data available"]
    #[inline(always)]
    pub fn is_jmbin1fg_1(&self) -> bool {
        *self == Jmbin1fg::Jmbin1fg1
    }
}
#[doc = "Field `JMBIN1FG` writer - Incoming JTAG Mailbox 1 flag"]
pub type Jmbin1fgW<'a, REG> = crate::BitWriter<'a, REG, Jmbin1fg>;
impl<'a, REG> Jmbin1fgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JMBI1 has no new data"]
    #[inline(always)]
    pub fn jmbin1fg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbin1fg::Jmbin1fg0)
    }
    #[doc = "JMBI1 has new data available"]
    #[inline(always)]
    pub fn jmbin1fg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbin1fg::Jmbin1fg1)
    }
}
#[doc = "Outgoing JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbout0fg {
    #[doc = "0: JMBO0 is not ready to receive new data"]
    Jmbout0fg0 = 0,
    #[doc = "1: JMBO0 is ready to receive new data"]
    Jmbout0fg1 = 1,
}
impl From<Jmbout0fg> for bool {
    #[inline(always)]
    fn from(variant: Jmbout0fg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBOUT0FG` reader - Outgoing JTAG Mailbox 0 flag"]
pub type Jmbout0fgR = crate::BitReader<Jmbout0fg>;
impl Jmbout0fgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbout0fg {
        match self.bits {
            false => Jmbout0fg::Jmbout0fg0,
            true => Jmbout0fg::Jmbout0fg1,
        }
    }
    #[doc = "JMBO0 is not ready to receive new data"]
    #[inline(always)]
    pub fn is_jmbout0fg_0(&self) -> bool {
        *self == Jmbout0fg::Jmbout0fg0
    }
    #[doc = "JMBO0 is ready to receive new data"]
    #[inline(always)]
    pub fn is_jmbout0fg_1(&self) -> bool {
        *self == Jmbout0fg::Jmbout0fg1
    }
}
#[doc = "Outgoing JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbout1fg {
    #[doc = "0: JMBO1 is not ready to receive new data"]
    Jmbout1fg0 = 0,
    #[doc = "1: JMBO1 is ready to receive new data"]
    Jmbout1fg1 = 1,
}
impl From<Jmbout1fg> for bool {
    #[inline(always)]
    fn from(variant: Jmbout1fg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBOUT1FG` reader - Outgoing JTAG Mailbox 1 flag"]
pub type Jmbout1fgR = crate::BitReader<Jmbout1fg>;
impl Jmbout1fgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbout1fg {
        match self.bits {
            false => Jmbout1fg::Jmbout1fg0,
            true => Jmbout1fg::Jmbout1fg1,
        }
    }
    #[doc = "JMBO1 is not ready to receive new data"]
    #[inline(always)]
    pub fn is_jmbout1fg_0(&self) -> bool {
        *self == Jmbout1fg::Jmbout1fg0
    }
    #[doc = "JMBO1 is ready to receive new data"]
    #[inline(always)]
    pub fn is_jmbout1fg_1(&self) -> bool {
        *self == Jmbout1fg::Jmbout1fg1
    }
}
#[doc = "Operation mode of JMB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbmode {
    #[doc = "0: 16-bit transfers using JMBO0 and JMBI0 only"]
    _16bit = 0,
    #[doc = "1: 32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    _32bit = 1,
}
impl From<Jmbmode> for bool {
    #[inline(always)]
    fn from(variant: Jmbmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBMODE` reader - Operation mode of JMB"]
pub type JmbmodeR = crate::BitReader<Jmbmode>;
impl JmbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbmode {
        match self.bits {
            false => Jmbmode::_16bit,
            true => Jmbmode::_32bit,
        }
    }
    #[doc = "16-bit transfers using JMBO0 and JMBI0 only"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Jmbmode::_16bit
    }
    #[doc = "32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == Jmbmode::_32bit
    }
}
#[doc = "Field `JMBMODE` writer - Operation mode of JMB"]
pub type JmbmodeW<'a, REG> = crate::BitWriter<'a, REG, Jmbmode>;
impl<'a, REG> JmbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit transfers using JMBO0 and JMBI0 only"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbmode::_16bit)
    }
    #[doc = "32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbmode::_32bit)
    }
}
#[doc = "Incoming JTAG Mailbox 0 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbclr0off {
    #[doc = "0: JMBIN0FG cleared on read of JMB0IN register"]
    Jmbclr0off0 = 0,
    #[doc = "1: JMBIN0FG cleared by software"]
    Jmbclr0off1 = 1,
}
impl From<Jmbclr0off> for bool {
    #[inline(always)]
    fn from(variant: Jmbclr0off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBCLR0OFF` reader - Incoming JTAG Mailbox 0 flag auto-clear disable"]
pub type Jmbclr0offR = crate::BitReader<Jmbclr0off>;
impl Jmbclr0offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbclr0off {
        match self.bits {
            false => Jmbclr0off::Jmbclr0off0,
            true => Jmbclr0off::Jmbclr0off1,
        }
    }
    #[doc = "JMBIN0FG cleared on read of JMB0IN register"]
    #[inline(always)]
    pub fn is_jmbclr0off_0(&self) -> bool {
        *self == Jmbclr0off::Jmbclr0off0
    }
    #[doc = "JMBIN0FG cleared by software"]
    #[inline(always)]
    pub fn is_jmbclr0off_1(&self) -> bool {
        *self == Jmbclr0off::Jmbclr0off1
    }
}
#[doc = "Field `JMBCLR0OFF` writer - Incoming JTAG Mailbox 0 flag auto-clear disable"]
pub type Jmbclr0offW<'a, REG> = crate::BitWriter<'a, REG, Jmbclr0off>;
impl<'a, REG> Jmbclr0offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JMBIN0FG cleared on read of JMB0IN register"]
    #[inline(always)]
    pub fn jmbclr0off_0(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbclr0off::Jmbclr0off0)
    }
    #[doc = "JMBIN0FG cleared by software"]
    #[inline(always)]
    pub fn jmbclr0off_1(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbclr0off::Jmbclr0off1)
    }
}
#[doc = "Incoming JTAG Mailbox 1 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jmbclr1off {
    #[doc = "0: JMBIN1FG cleared on read of JMB1IN register"]
    Jmbclr1off0 = 0,
    #[doc = "1: JMBIN1FG cleared by software"]
    Jmbclr1off1 = 1,
}
impl From<Jmbclr1off> for bool {
    #[inline(always)]
    fn from(variant: Jmbclr1off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JMBCLR1OFF` reader - Incoming JTAG Mailbox 1 flag auto-clear disable"]
pub type Jmbclr1offR = crate::BitReader<Jmbclr1off>;
impl Jmbclr1offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jmbclr1off {
        match self.bits {
            false => Jmbclr1off::Jmbclr1off0,
            true => Jmbclr1off::Jmbclr1off1,
        }
    }
    #[doc = "JMBIN1FG cleared on read of JMB1IN register"]
    #[inline(always)]
    pub fn is_jmbclr1off_0(&self) -> bool {
        *self == Jmbclr1off::Jmbclr1off0
    }
    #[doc = "JMBIN1FG cleared by software"]
    #[inline(always)]
    pub fn is_jmbclr1off_1(&self) -> bool {
        *self == Jmbclr1off::Jmbclr1off1
    }
}
#[doc = "Field `JMBCLR1OFF` writer - Incoming JTAG Mailbox 1 flag auto-clear disable"]
pub type Jmbclr1offW<'a, REG> = crate::BitWriter<'a, REG, Jmbclr1off>;
impl<'a, REG> Jmbclr1offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JMBIN1FG cleared on read of JMB1IN register"]
    #[inline(always)]
    pub fn jmbclr1off_0(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbclr1off::Jmbclr1off0)
    }
    #[doc = "JMBIN1FG cleared by software"]
    #[inline(always)]
    pub fn jmbclr1off_1(self) -> &'a mut crate::W<REG> {
        self.variant(Jmbclr1off::Jmbclr1off1)
    }
}
impl R {
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> Jmbin0fgR {
        Jmbin0fgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> Jmbin1fgR {
        Jmbin1fgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Outgoing JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> Jmbout0fgR {
        Jmbout0fgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Outgoing JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> Jmbout1fgR {
        Jmbout1fgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JmbmodeR {
        JmbmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> Jmbclr0offR {
        Jmbclr0offR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> Jmbclr1offR {
        Jmbclr1offR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbin0fg(&mut self) -> Jmbin0fgW<'_, SysjmbcSpec> {
        Jmbin0fgW::new(self, 0)
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbin1fg(&mut self) -> Jmbin1fgW<'_, SysjmbcSpec> {
        Jmbin1fgW::new(self, 1)
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    pub fn jmbmode(&mut self) -> JmbmodeW<'_, SysjmbcSpec> {
        JmbmodeW::new(self, 4)
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr0off(&mut self) -> Jmbclr0offW<'_, SysjmbcSpec> {
        Jmbclr0offW::new(self, 6)
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr1off(&mut self) -> Jmbclr1offW<'_, SysjmbcSpec> {
        Jmbclr1offW::new(self, 7)
    }
}
#[doc = "JTAG Mailbox Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysjmbcSpec;
impl crate::RegisterSpec for SysjmbcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbc::R`](R) reader structure"]
impl crate::Readable for SysjmbcSpec {}
#[doc = "`write(|w| ..)` method takes [`sysjmbc::W`](W) writer structure"]
impl crate::Writable for SysjmbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSJMBC to value 0"]
impl crate::Resettable for SysjmbcSpec {}
