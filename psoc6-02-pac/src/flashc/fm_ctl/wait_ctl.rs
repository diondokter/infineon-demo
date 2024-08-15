#[doc = "Register `WAIT_CTL` reader"]
pub type R = crate::R<WaitCtlSpec>;
#[doc = "Register `WAIT_CTL` writer"]
pub type W = crate::W<WaitCtlSpec>;
#[doc = "Field `WAIT_FM_MEM_RD` reader - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
pub type WaitFmMemRdR = crate::FieldReader;
#[doc = "Field `WAIT_FM_MEM_RD` writer - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
pub type WaitFmMemRdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAIT_FM_HV_RD` reader - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
pub type WaitFmHvRdR = crate::FieldReader;
#[doc = "Field `WAIT_FM_HV_RD` writer - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
pub type WaitFmHvRdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAIT_FM_HV_WR` reader - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
pub type WaitFmHvWrR = crate::FieldReader;
#[doc = "Field `WAIT_FM_HV_WR` writer - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
pub type WaitFmHvWrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FM_RWW_MODE` reader - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
pub type FmRwwModeR = crate::FieldReader;
#[doc = "Field `FM_RWW_MODE` writer - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
pub type FmRwwModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LV_SPARE_1` reader - Spare register"]
pub type LvSpare1R = crate::BitReader;
#[doc = "Field `LV_SPARE_1` writer - Spare register"]
pub type LvSpare1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRMM` reader - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
pub type DrmmR = crate::BitReader;
#[doc = "Field `DRMM` writer - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
pub type DrmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBA` reader - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
pub type MbaR = crate::BitReader;
#[doc = "Field `MBA` writer - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
pub type MbaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL_SOFT_SET_EN` reader - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
pub type PlSoftSetEnR = crate::BitReader;
#[doc = "Field `PL_SOFT_SET_EN` writer - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
pub type PlSoftSetEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&self) -> WaitFmMemRdR {
        WaitFmMemRdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&self) -> WaitFmHvRdR {
        WaitFmHvRdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&self) -> WaitFmHvWrR {
        WaitFmHvWrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    pub fn fm_rww_mode(&self) -> FmRwwModeR {
        FmRwwModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Spare register"]
    #[inline(always)]
    pub fn lv_spare_1(&self) -> LvSpare1R {
        LvSpare1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    pub fn drmm(&self) -> DrmmR {
        DrmmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    pub fn mba(&self) -> MbaR {
        MbaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    pub fn pl_soft_set_en(&self) -> PlSoftSetEnR {
        PlSoftSetEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    #[must_use]
    pub fn wait_fm_mem_rd(&mut self) -> WaitFmMemRdW<WaitCtlSpec> {
        WaitFmMemRdW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    #[must_use]
    pub fn wait_fm_hv_rd(&mut self) -> WaitFmHvRdW<WaitCtlSpec> {
        WaitFmHvRdW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    #[must_use]
    pub fn wait_fm_hv_wr(&mut self) -> WaitFmHvWrW<WaitCtlSpec> {
        WaitFmHvWrW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    #[must_use]
    pub fn fm_rww_mode(&mut self) -> FmRwwModeW<WaitCtlSpec> {
        FmRwwModeW::new(self, 24)
    }
    #[doc = "Bit 26 - Spare register"]
    #[inline(always)]
    #[must_use]
    pub fn lv_spare_1(&mut self) -> LvSpare1W<WaitCtlSpec> {
        LvSpare1W::new(self, 26)
    }
    #[doc = "Bit 27 - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    #[must_use]
    pub fn drmm(&mut self) -> DrmmW<WaitCtlSpec> {
        DrmmW::new(self, 27)
    }
    #[doc = "Bit 28 - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    #[must_use]
    pub fn mba(&mut self) -> MbaW<WaitCtlSpec> {
        MbaW::new(self, 28)
    }
    #[doc = "Bit 29 - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    #[must_use]
    pub fn pl_soft_set_en(&mut self) -> PlSoftSetEnW<WaitCtlSpec> {
        PlSoftSetEnW::new(self, 29)
    }
}
#[doc = "Wait State control\n\nYou can [`read`](crate::Reg::read) this register and get [`wait_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wait_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaitCtlSpec;
impl crate::RegisterSpec for WaitCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wait_ctl::R`](R) reader structure"]
impl crate::Readable for WaitCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`wait_ctl::W`](W) writer structure"]
impl crate::Writable for WaitCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAIT_CTL to value 0x0003_0b09"]
impl crate::Resettable for WaitCtlSpec {
    const RESET_VALUE: u32 = 0x0003_0b09;
}
