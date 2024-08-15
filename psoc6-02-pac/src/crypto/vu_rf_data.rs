#[doc = "Register `VU_RF_DATA[%s]` reader"]
pub type R = crate::R<VuRfDataSpec>;
#[doc = "Field `DATA32` reader - Vector unit register-file data. A register-file register has the following layout: DATA\\[28:16\\]: data (typically used as a word offset in vector unit operand memory). DATA\\[12:0\\]: bit size minus 1."]
pub type Data32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Vector unit register-file data. A register-file register has the following layout: DATA\\[28:16\\]: data (typically used as a word offset in vector unit operand memory). DATA\\[12:0\\]: bit size minus 1."]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
#[doc = "Vector unit register-file\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_rf_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VuRfDataSpec;
impl crate::RegisterSpec for VuRfDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vu_rf_data::R`](R) reader structure"]
impl crate::Readable for VuRfDataSpec {}
#[doc = "`reset()` method sets VU_RF_DATA[%s]
to value 0"]
impl crate::Resettable for VuRfDataSpec {
    const RESET_VALUE: u32 = 0;
}
