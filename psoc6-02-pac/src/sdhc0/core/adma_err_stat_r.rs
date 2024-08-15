#[doc = "Register `ADMA_ERR_STAT_R` reader"]
pub type R = crate::R<AdmaErrStatRSpec>;
#[doc = "Field `ADMA_ERR_STATES` reader - ADMA Error States These bits indicate the state of ADMA when an error occurs during ADMA data transfer. Values: - 0x0 (ST_STOP): Stop DMA - SYS_ADR register points to a location next to the error descriptor - 0x1 (ST_FDS): Fetch Descriptor - SYS_ADR register points to the error descriptor - 0x2 (UNUSED): Never set this state - 0x3 (ST_TFR): Transfer Data - SYS_ADR register points to a location next to the error descriptor"]
pub type AdmaErrStatesR = crate::FieldReader;
#[doc = "Field `ADMA_LEN_ERR` reader - ADMA Length Mismatch Error States This error occurs in the following instances: - While the Block Count Enable is being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length - When the total data length cannot be divided by the block length Values: - 0x0 (NO_ERR): No Error - 0x1 (ERROR): Error"]
pub type AdmaLenErrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - ADMA Error States These bits indicate the state of ADMA when an error occurs during ADMA data transfer. Values: - 0x0 (ST_STOP): Stop DMA - SYS_ADR register points to a location next to the error descriptor - 0x1 (ST_FDS): Fetch Descriptor - SYS_ADR register points to the error descriptor - 0x2 (UNUSED): Never set this state - 0x3 (ST_TFR): Transfer Data - SYS_ADR register points to a location next to the error descriptor"]
    #[inline(always)]
    pub fn adma_err_states(&self) -> AdmaErrStatesR {
        AdmaErrStatesR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error States This error occurs in the following instances: - While the Block Count Enable is being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length - When the total data length cannot be divided by the block length Values: - 0x0 (NO_ERR): No Error - 0x1 (ERROR): Error"]
    #[inline(always)]
    pub fn adma_len_err(&self) -> AdmaLenErrR {
        AdmaLenErrR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_err_stat_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaErrStatRSpec;
impl crate::RegisterSpec for AdmaErrStatRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adma_err_stat_r::R`](R) reader structure"]
impl crate::Readable for AdmaErrStatRSpec {}
#[doc = "`reset()` method sets ADMA_ERR_STAT_R to value 0"]
impl crate::Resettable for AdmaErrStatRSpec {
    const RESET_VALUE: u8 = 0;
}
