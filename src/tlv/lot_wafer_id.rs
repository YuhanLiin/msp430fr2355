#[doc = "Register `LOT_WAFER_ID` reader"]
pub type R = crate::R<LotWaferIdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Lot/wafer ID\n\nYou can [`read`](crate::Reg::read) this register and get [`lot_wafer_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LotWaferIdSpec;
impl crate::RegisterSpec for LotWaferIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lot_wafer_id::R`](R) reader structure"]
impl crate::Readable for LotWaferIdSpec {}
#[doc = "`reset()` method sets LOT_WAFER_ID to value 0"]
impl crate::Resettable for LotWaferIdSpec {}
