#[doc = "Register `UCB0ADDMASK` reader"]
pub type R = crate::R<Ucb0addmaskSpec>;
#[doc = "Register `UCB0ADDMASK` writer"]
pub type W = crate::W<Ucb0addmaskSpec>;
#[doc = "Field `ADDMASK` reader - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type AddmaskR = crate::FieldReader<u16>;
#[doc = "Field `ADDMASK` writer - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type AddmaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&self) -> AddmaskR {
        AddmaskR::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&mut self) -> AddmaskW<'_, Ucb0addmaskSpec> {
        AddmaskW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0addmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0addmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0addmaskSpec;
impl crate::RegisterSpec for Ucb0addmaskSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0addmask::R`](R) reader structure"]
impl crate::Readable for Ucb0addmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0addmask::W`](W) writer structure"]
impl crate::Writable for Ucb0addmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0ADDMASK to value 0"]
impl crate::Resettable for Ucb0addmaskSpec {}
