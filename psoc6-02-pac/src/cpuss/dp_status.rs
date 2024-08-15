#[doc = "Register `DP_STATUS` reader"]
pub type R = crate::R<DpStatusSpec>;
#[doc = "Field `SWJ_CONNECTED` reader - Specifies if the SWJ debug port is connected; i.e. debug host interface is active: '0': Not connected/not active. '1': Connected/active."]
pub type SwjConnectedR = crate::BitReader;
#[doc = "Field `SWJ_DEBUG_EN` reader - Specifies if SWJ debug is enabled, i.e. CDBGPWRUPACK is '1' and thus debug clocks are on: '0': Disabled. '1': Enabled."]
pub type SwjDebugEnR = crate::BitReader;
#[doc = "Field `SWJ_JTAG_SEL` reader - Specifies if the JTAG or SWD interface is selected. This signal is valid when DP_CTL.PTM_SEL is '0' (SWJ mode selected) and SWJ_CONNECTED is '1' (SWJ is connected). '0': SWD selected. '1': JTAG selected."]
pub type SwjJtagSelR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Specifies if the SWJ debug port is connected; i.e. debug host interface is active: '0': Not connected/not active. '1': Connected/active."]
    #[inline(always)]
    pub fn swj_connected(&self) -> SwjConnectedR {
        SwjConnectedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies if SWJ debug is enabled, i.e. CDBGPWRUPACK is '1' and thus debug clocks are on: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn swj_debug_en(&self) -> SwjDebugEnR {
        SwjDebugEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specifies if the JTAG or SWD interface is selected. This signal is valid when DP_CTL.PTM_SEL is '0' (SWJ mode selected) and SWJ_CONNECTED is '1' (SWJ is connected). '0': SWD selected. '1': JTAG selected."]
    #[inline(always)]
    pub fn swj_jtag_sel(&self) -> SwjJtagSelR {
        SwjJtagSelR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Debug port status\n\nYou can [`read`](crate::Reg::read) this register and get [`dp_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpStatusSpec;
impl crate::RegisterSpec for DpStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_status::R`](R) reader structure"]
impl crate::Readable for DpStatusSpec {}
#[doc = "`reset()` method sets DP_STATUS to value 0x04"]
impl crate::Resettable for DpStatusSpec {
    const RESET_VALUE: u32 = 0x04;
}
