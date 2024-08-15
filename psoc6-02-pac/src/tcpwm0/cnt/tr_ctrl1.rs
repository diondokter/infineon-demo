#[doc = "Register `TR_CTRL1` reader"]
pub type R = crate::R<TrCtrl1Spec>;
#[doc = "Register `TR_CTRL1` writer"]
pub type W = crate::W<TrCtrl1Spec>;
#[doc = "A capture event will copy the counter value into the CC register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CaptureEdge {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RisingEdge = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FallingEdge = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BothEdges = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NoEdgeDet = 3,
}
impl From<CaptureEdge> for u8 {
    #[inline(always)]
    fn from(variant: CaptureEdge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CaptureEdge {
    type Ux = u8;
}
impl crate::IsEnum for CaptureEdge {}
#[doc = "Field `CAPTURE_EDGE` reader - A capture event will copy the counter value into the CC register."]
pub type CaptureEdgeR = crate::FieldReader<CaptureEdge>;
impl CaptureEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CaptureEdge {
        match self.bits {
            0 => CaptureEdge::RisingEdge,
            1 => CaptureEdge::FallingEdge,
            2 => CaptureEdge::BothEdges,
            3 => CaptureEdge::NoEdgeDet,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CaptureEdge::RisingEdge
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CaptureEdge::FallingEdge
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == CaptureEdge::BothEdges
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == CaptureEdge::NoEdgeDet
    }
}
#[doc = "Field `CAPTURE_EDGE` writer - A capture event will copy the counter value into the CC register."]
pub type CaptureEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CaptureEdge, crate::Safe>;
impl<'a, REG> CaptureEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CaptureEdge::RisingEdge)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CaptureEdge::FallingEdge)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(CaptureEdge::BothEdges)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut crate::W<REG> {
        self.variant(CaptureEdge::NoEdgeDet)
    }
}
#[doc = "A counter event will increase or decrease the counter by '1'.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CountEdge {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RisingEdge = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FallingEdge = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BothEdges = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NoEdgeDet = 3,
}
impl From<CountEdge> for u8 {
    #[inline(always)]
    fn from(variant: CountEdge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CountEdge {
    type Ux = u8;
}
impl crate::IsEnum for CountEdge {}
#[doc = "Field `COUNT_EDGE` reader - A counter event will increase or decrease the counter by '1'."]
pub type CountEdgeR = crate::FieldReader<CountEdge>;
impl CountEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CountEdge {
        match self.bits {
            0 => CountEdge::RisingEdge,
            1 => CountEdge::FallingEdge,
            2 => CountEdge::BothEdges,
            3 => CountEdge::NoEdgeDet,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CountEdge::RisingEdge
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CountEdge::FallingEdge
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == CountEdge::BothEdges
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == CountEdge::NoEdgeDet
    }
}
#[doc = "Field `COUNT_EDGE` writer - A counter event will increase or decrease the counter by '1'."]
pub type CountEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CountEdge, crate::Safe>;
impl<'a, REG> CountEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CountEdge::RisingEdge)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CountEdge::FallingEdge)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(CountEdge::BothEdges)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut crate::W<REG> {
        self.variant(CountEdge::NoEdgeDet)
    }
}
#[doc = "A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReloadEdge {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RisingEdge = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FallingEdge = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BothEdges = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NoEdgeDet = 3,
}
impl From<ReloadEdge> for u8 {
    #[inline(always)]
    fn from(variant: ReloadEdge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReloadEdge {
    type Ux = u8;
}
impl crate::IsEnum for ReloadEdge {}
#[doc = "Field `RELOAD_EDGE` reader - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
pub type ReloadEdgeR = crate::FieldReader<ReloadEdge>;
impl ReloadEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReloadEdge {
        match self.bits {
            0 => ReloadEdge::RisingEdge,
            1 => ReloadEdge::FallingEdge,
            2 => ReloadEdge::BothEdges,
            3 => ReloadEdge::NoEdgeDet,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ReloadEdge::RisingEdge
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ReloadEdge::FallingEdge
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == ReloadEdge::BothEdges
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == ReloadEdge::NoEdgeDet
    }
}
#[doc = "Field `RELOAD_EDGE` writer - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
pub type ReloadEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ReloadEdge, crate::Safe>;
impl<'a, REG> ReloadEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ReloadEdge::RisingEdge)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ReloadEdge::FallingEdge)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(ReloadEdge::BothEdges)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut crate::W<REG> {
        self.variant(ReloadEdge::NoEdgeDet)
    }
}
#[doc = "A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StopEdge {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RisingEdge = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FallingEdge = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BothEdges = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NoEdgeDet = 3,
}
impl From<StopEdge> for u8 {
    #[inline(always)]
    fn from(variant: StopEdge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StopEdge {
    type Ux = u8;
}
impl crate::IsEnum for StopEdge {}
#[doc = "Field `STOP_EDGE` reader - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
pub type StopEdgeR = crate::FieldReader<StopEdge>;
impl StopEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopEdge {
        match self.bits {
            0 => StopEdge::RisingEdge,
            1 => StopEdge::FallingEdge,
            2 => StopEdge::BothEdges,
            3 => StopEdge::NoEdgeDet,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == StopEdge::RisingEdge
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == StopEdge::FallingEdge
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == StopEdge::BothEdges
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == StopEdge::NoEdgeDet
    }
}
#[doc = "Field `STOP_EDGE` writer - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
pub type StopEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, StopEdge, crate::Safe>;
impl<'a, REG> StopEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(StopEdge::RisingEdge)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(StopEdge::FallingEdge)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(StopEdge::BothEdges)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut crate::W<REG> {
        self.variant(StopEdge::NoEdgeDet)
    }
}
#[doc = "A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StartEdge {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RisingEdge = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FallingEdge = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BothEdges = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NoEdgeDet = 3,
}
impl From<StartEdge> for u8 {
    #[inline(always)]
    fn from(variant: StartEdge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StartEdge {
    type Ux = u8;
}
impl crate::IsEnum for StartEdge {}
#[doc = "Field `START_EDGE` reader - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
pub type StartEdgeR = crate::FieldReader<StartEdge>;
impl StartEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StartEdge {
        match self.bits {
            0 => StartEdge::RisingEdge,
            1 => StartEdge::FallingEdge,
            2 => StartEdge::BothEdges,
            3 => StartEdge::NoEdgeDet,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == StartEdge::RisingEdge
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == StartEdge::FallingEdge
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == StartEdge::BothEdges
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == StartEdge::NoEdgeDet
    }
}
#[doc = "Field `START_EDGE` writer - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
pub type StartEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, StartEdge, crate::Safe>;
impl<'a, REG> StartEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(StartEdge::RisingEdge)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(StartEdge::FallingEdge)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(StartEdge::BothEdges)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut crate::W<REG> {
        self.variant(StartEdge::NoEdgeDet)
    }
}
impl R {
    #[doc = "Bits 0:1 - A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub fn capture_edge(&self) -> CaptureEdgeR {
        CaptureEdgeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn count_edge(&self) -> CountEdgeR {
        CountEdgeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn reload_edge(&self) -> ReloadEdgeR {
        ReloadEdgeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn stop_edge(&self) -> StopEdgeR {
        StopEdgeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn start_edge(&self) -> StartEdgeR {
        StartEdgeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    #[must_use]
    pub fn capture_edge(&mut self) -> CaptureEdgeW<TrCtrl1Spec> {
        CaptureEdgeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    #[must_use]
    pub fn count_edge(&mut self) -> CountEdgeW<TrCtrl1Spec> {
        CountEdgeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    #[must_use]
    pub fn reload_edge(&mut self) -> ReloadEdgeW<TrCtrl1Spec> {
        ReloadEdgeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    #[must_use]
    pub fn stop_edge(&mut self) -> StopEdgeW<TrCtrl1Spec> {
        StopEdgeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    #[must_use]
    pub fn start_edge(&mut self) -> StartEdgeW<TrCtrl1Spec> {
        StartEdgeW::new(self, 8)
    }
}
#[doc = "Counter trigger control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtrl1Spec;
impl crate::RegisterSpec for TrCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctrl1::R`](R) reader structure"]
impl crate::Readable for TrCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctrl1::W`](W) writer structure"]
impl crate::Writable for TrCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTRL1 to value 0x03ff"]
impl crate::Resettable for TrCtrl1Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
