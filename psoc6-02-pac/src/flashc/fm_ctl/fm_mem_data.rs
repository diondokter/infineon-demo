#[doc = "Register `FM_MEM_DATA[%s]` reader"]
pub type R = crate::R<FmMemDataSpec>;
#[doc = "Field `DATA32` reader - Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is 0: data as specified by the R interface address - IF_SEL is 1: data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
pub type Data32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is 0: data as specified by the R interface address - IF_SEL is 1: data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
#[doc = "Flash macro memory sense amplifier and column decoder data\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_mem_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmMemDataSpec;
impl crate::RegisterSpec for FmMemDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_mem_data::R`](R) reader structure"]
impl crate::Readable for FmMemDataSpec {}
#[doc = "`reset()` method sets FM_MEM_DATA[%s]
to value 0"]
impl crate::Resettable for FmMemDataSpec {
    const RESET_VALUE: u32 = 0;
}
