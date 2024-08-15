#[doc = "Register `CLK_CAL_CNT2` reader"]
pub type R = crate::R<ClkCalCnt2Spec>;
#[doc = "Field `CAL_COUNTER2` reader - Up-counter clocked on fast clock output #1 (see CLK_OUTPUT_FAST). When CLK_CAL_CNT1.CAL_COUNTER_DONE==1, the counter is stopped and can be read by SW. Do not read this value unless CAL_COUNTER_DONE==1. The expected final value is related to the ratio of clock frequencies used for the two counters and the value loaded into counter 1: CLK_CAL_CNT2.COUNTER=(F_cnt2/F_cnt1)*(CLK_CAL_CNT1.COUNTER)"]
pub type CalCounter2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Up-counter clocked on fast clock output #1 (see CLK_OUTPUT_FAST). When CLK_CAL_CNT1.CAL_COUNTER_DONE==1, the counter is stopped and can be read by SW. Do not read this value unless CAL_COUNTER_DONE==1. The expected final value is related to the ratio of clock frequencies used for the two counters and the value loaded into counter 1: CLK_CAL_CNT2.COUNTER=(F_cnt2/F_cnt1)*(CLK_CAL_CNT1.COUNTER)"]
    #[inline(always)]
    pub fn cal_counter2(&self) -> CalCounter2R {
        CalCounter2R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Clock Calibration Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cal_cnt2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCalCnt2Spec;
impl crate::RegisterSpec for ClkCalCnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cal_cnt2::R`](R) reader structure"]
impl crate::Readable for ClkCalCnt2Spec {}
#[doc = "`reset()` method sets CLK_CAL_CNT2 to value 0"]
impl crate::Resettable for ClkCalCnt2Spec {
    const RESET_VALUE: u32 = 0;
}
