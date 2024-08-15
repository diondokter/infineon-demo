#[doc = "Register `CLK_CAL_CNT1` reader"]
pub type R = crate::R<ClkCalCnt1Spec>;
#[doc = "Register `CLK_CAL_CNT1` writer"]
pub type W = crate::W<ClkCalCnt1Spec>;
#[doc = "Field `CAL_COUNTER1` reader - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
pub type CalCounter1R = crate::FieldReader<u32>;
#[doc = "Field `CAL_COUNTER1` writer - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
pub type CalCounter1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CAL_COUNTER_DONE` reader - Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
pub type CalCounterDoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    pub fn cal_counter1(&self) -> CalCounter1R {
        CalCounter1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
    #[inline(always)]
    pub fn cal_counter_done(&self) -> CalCounterDoneR {
        CalCounterDoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    #[must_use]
    pub fn cal_counter1(&mut self) -> CalCounter1W<ClkCalCnt1Spec> {
        CalCounter1W::new(self, 0)
    }
}
#[doc = "Clock Calibration Counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cal_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cal_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCalCnt1Spec;
impl crate::RegisterSpec for ClkCalCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cal_cnt1::R`](R) reader structure"]
impl crate::Readable for ClkCalCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_cal_cnt1::W`](W) writer structure"]
impl crate::Writable for ClkCalCnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CAL_CNT1 to value 0x8000_0000"]
impl crate::Resettable for ClkCalCnt1Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
