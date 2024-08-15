#[doc = "Register `EMMC_CTRL_R` reader"]
pub type R = crate::R<EmmcCtrlRSpec>;
#[doc = "Register `EMMC_CTRL_R` writer"]
pub type W = crate::W<EmmcCtrlRSpec>;
#[doc = "Field `CARD_IS_EMMC` reader - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
pub type CardIsEmmcR = crate::BitReader;
#[doc = "Field `CARD_IS_EMMC` writer - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
pub type CardIsEmmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_DATA_CRC_CHK` reader - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
pub type DisableDataCrcChkR = crate::BitReader;
#[doc = "Field `DISABLE_DATA_CRC_CHK` writer - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
pub type DisableDataCrcChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMMC_RST_N` reader - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
pub type EmmcRstNR = crate::BitReader;
#[doc = "Field `EMMC_RST_N` writer - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
pub type EmmcRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMMC_RST_N_OE` reader - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
pub type EmmcRstNOeR = crate::BitReader;
#[doc = "Field `EMMC_RST_N_OE` writer - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
pub type EmmcRstNOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQE_ALGO_SEL` reader - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
pub type CqeAlgoSelR = crate::BitReader;
#[doc = "Field `CQE_ALGO_SEL` writer - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
pub type CqeAlgoSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQE_PREFETCH_DISABLE` reader - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
pub type CqePrefetchDisableR = crate::BitReader;
#[doc = "Field `CQE_PREFETCH_DISABLE` writer - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
pub type CqePrefetchDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
    #[inline(always)]
    pub fn card_is_emmc(&self) -> CardIsEmmcR {
        CardIsEmmcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    pub fn disable_data_crc_chk(&self) -> DisableDataCrcChkR {
        DisableDataCrcChkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    pub fn emmc_rst_n(&self) -> EmmcRstNR {
        EmmcRstNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
    #[inline(always)]
    pub fn emmc_rst_n_oe(&self) -> EmmcRstNOeR {
        EmmcRstNOeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    pub fn cqe_algo_sel(&self) -> CqeAlgoSelR {
        CqeAlgoSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    pub fn cqe_prefetch_disable(&self) -> CqePrefetchDisableR {
        CqePrefetchDisableR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
    #[inline(always)]
    #[must_use]
    pub fn card_is_emmc(&mut self) -> CardIsEmmcW<EmmcCtrlRSpec> {
        CardIsEmmcW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn disable_data_crc_chk(&mut self) -> DisableDataCrcChkW<EmmcCtrlRSpec> {
        DisableDataCrcChkW::new(self, 1)
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_rst_n(&mut self) -> EmmcRstNW<EmmcCtrlRSpec> {
        EmmcRstNW::new(self, 2)
    }
    #[doc = "Bit 3 - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_rst_n_oe(&mut self) -> EmmcRstNOeW<EmmcCtrlRSpec> {
        EmmcRstNOeW::new(self, 3)
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_algo_sel(&mut self) -> CqeAlgoSelW<EmmcCtrlRSpec> {
        CqeAlgoSelW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_prefetch_disable(&mut self) -> CqePrefetchDisableW<EmmcCtrlRSpec> {
        CqePrefetchDisableW::new(self, 10)
    }
}
#[doc = "eMMC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`emmc_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emmc_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmcCtrlRSpec;
impl crate::RegisterSpec for EmmcCtrlRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmc_ctrl_r::R`](R) reader structure"]
impl crate::Readable for EmmcCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`emmc_ctrl_r::W`](W) writer structure"]
impl crate::Writable for EmmcCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMC_CTRL_R to value 0x0c"]
impl crate::Resettable for EmmcCtrlRSpec {
    const RESET_VALUE: u16 = 0x0c;
}
