#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `AUTO_RELOAD_CC` reader - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
pub type AutoReloadCcR = crate::BitReader;
#[doc = "Field `AUTO_RELOAD_CC` writer - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
pub type AutoReloadCcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_RELOAD_PERIOD` reader - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
pub type AutoReloadPeriodR = crate::BitReader;
#[doc = "Field `AUTO_RELOAD_PERIOD` writer - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
pub type AutoReloadPeriodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_SYNC_KILL` reader - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
pub type PwmSyncKillR = crate::BitReader;
#[doc = "Field `PWM_SYNC_KILL` writer - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
pub type PwmSyncKillW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_STOP_ON_KILL` reader - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
pub type PwmStopOnKillR = crate::BitReader;
#[doc = "Field `PWM_STOP_ON_KILL` writer - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
pub type PwmStopOnKillW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENERIC` reader - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
pub type GenericR = crate::FieldReader;
#[doc = "Field `GENERIC` writer - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
pub type GenericW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Determines counter direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UpDownMode {
    #[doc = "0: Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    CountUp = 0,
    #[doc = "1: Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    CountDown = 1,
    #[doc = "2: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    CountUpdn1 = 2,
    #[doc = "3: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    CountUpdn2 = 3,
}
impl From<UpDownMode> for u8 {
    #[inline(always)]
    fn from(variant: UpDownMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UpDownMode {
    type Ux = u8;
}
impl crate::IsEnum for UpDownMode {}
#[doc = "Field `UP_DOWN_MODE` reader - Determines counter direction."]
pub type UpDownModeR = crate::FieldReader<UpDownMode>;
impl UpDownModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UpDownMode {
        match self.bits {
            0 => UpDownMode::CountUp,
            1 => UpDownMode::CountDown,
            2 => UpDownMode::CountUpdn1,
            3 => UpDownMode::CountUpdn2,
            _ => unreachable!(),
        }
    }
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    #[inline(always)]
    pub fn is_count_up(&self) -> bool {
        *self == UpDownMode::CountUp
    }
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn is_count_down(&self) -> bool {
        *self == UpDownMode::CountDown
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn is_count_updn1(&self) -> bool {
        *self == UpDownMode::CountUpdn1
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    #[inline(always)]
    pub fn is_count_updn2(&self) -> bool {
        *self == UpDownMode::CountUpdn2
    }
}
#[doc = "Field `UP_DOWN_MODE` writer - Determines counter direction."]
pub type UpDownModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, UpDownMode, crate::Safe>;
impl<'a, REG> UpDownModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    #[inline(always)]
    pub fn count_up(self) -> &'a mut crate::W<REG> {
        self.variant(UpDownMode::CountUp)
    }
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_down(self) -> &'a mut crate::W<REG> {
        self.variant(UpDownMode::CountDown)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_updn1(self) -> &'a mut crate::W<REG> {
        self.variant(UpDownMode::CountUpdn1)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    #[inline(always)]
    pub fn count_updn2(self) -> &'a mut crate::W<REG> {
        self.variant(UpDownMode::CountUpdn2)
    }
}
#[doc = "Field `ONE_SHOT` reader - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
pub type OneShotR = crate::BitReader;
#[doc = "Field `ONE_SHOT` writer - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
pub type OneShotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QuadratureMode {
    #[doc = "0: X1 encoding (QUAD mode)"]
    X1 = 0,
    #[doc = "1: X2 encoding (QUAD mode)"]
    X2 = 1,
    #[doc = "2: X4 encoding (QUAD mode)"]
    X4 = 2,
}
impl From<QuadratureMode> for u8 {
    #[inline(always)]
    fn from(variant: QuadratureMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QuadratureMode {
    type Ux = u8;
}
impl crate::IsEnum for QuadratureMode {}
#[doc = "Field `QUADRATURE_MODE` reader - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
pub type QuadratureModeR = crate::FieldReader<QuadratureMode>;
impl QuadratureModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<QuadratureMode> {
        match self.bits {
            0 => Some(QuadratureMode::X1),
            1 => Some(QuadratureMode::X2),
            2 => Some(QuadratureMode::X4),
            _ => None,
        }
    }
    #[doc = "X1 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == QuadratureMode::X1
    }
    #[doc = "X2 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == QuadratureMode::X2
    }
    #[doc = "X4 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == QuadratureMode::X4
    }
}
#[doc = "Field `QUADRATURE_MODE` writer - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
pub type QuadratureModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, QuadratureMode>;
impl<'a, REG> QuadratureModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "X1 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(QuadratureMode::X1)
    }
    #[doc = "X2 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(QuadratureMode::X2)
    }
    #[doc = "X4 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(QuadratureMode::X4)
    }
}
#[doc = "Counter mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Timer mode"]
    Timer = 0,
    #[doc = "2: Capture mode"]
    Capture = 2,
    #[doc = "3: Quadrature encoding mode"]
    Quad = 3,
    #[doc = "4: Pulse width modulation (PWM) mode"]
    Pwm = 4,
    #[doc = "5: PWM with deadtime insertion mode"]
    PwmDt = 5,
    #[doc = "6: Pseudo random pulse width modulation"]
    PwmPr = 6,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Counter mode."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Timer),
            2 => Some(Mode::Capture),
            3 => Some(Mode::Quad),
            4 => Some(Mode::Pwm),
            5 => Some(Mode::PwmDt),
            6 => Some(Mode::PwmPr),
            _ => None,
        }
    }
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Mode::Timer
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Mode::Capture
    }
    #[doc = "Quadrature encoding mode"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Mode::Quad
    }
    #[doc = "Pulse width modulation (PWM) mode"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Mode::Pwm
    }
    #[doc = "PWM with deadtime insertion mode"]
    #[inline(always)]
    pub fn is_pwm_dt(&self) -> bool {
        *self == Mode::PwmDt
    }
    #[doc = "Pseudo random pulse width modulation"]
    #[inline(always)]
    pub fn is_pwm_pr(&self) -> bool {
        *self == Mode::PwmPr
    }
}
#[doc = "Field `MODE` writer - Counter mode."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Timer)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Capture)
    }
    #[doc = "Quadrature encoding mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Quad)
    }
    #[doc = "Pulse width modulation (PWM) mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Pwm)
    }
    #[doc = "PWM with deadtime insertion mode"]
    #[inline(always)]
    pub fn pwm_dt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PwmDt)
    }
    #[doc = "Pseudo random pulse width modulation"]
    #[inline(always)]
    pub fn pwm_pr(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PwmPr)
    }
}
impl R {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc(&self) -> AutoReloadCcR {
        AutoReloadCcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_period(&self) -> AutoReloadPeriodR {
        AutoReloadPeriodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn pwm_sync_kill(&self) -> PwmSyncKillR {
        PwmSyncKillR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&self) -> PwmStopOnKillR {
        PwmStopOnKillR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub fn generic(&self) -> GenericR {
        GenericR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    pub fn up_down_mode(&self) -> UpDownModeR {
        UpDownModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn one_shot(&self) -> OneShotR {
        OneShotR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub fn quadrature_mode(&self) -> QuadratureModeR {
        QuadratureModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    #[must_use]
    pub fn auto_reload_cc(&mut self) -> AutoReloadCcW<CtrlSpec> {
        AutoReloadCcW::new(self, 0)
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    #[must_use]
    pub fn auto_reload_period(&mut self) -> AutoReloadPeriodW<CtrlSpec> {
        AutoReloadPeriodW::new(self, 1)
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sync_kill(&mut self) -> PwmSyncKillW<CtrlSpec> {
        PwmSyncKillW::new(self, 2)
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_stop_on_kill(&mut self) -> PwmStopOnKillW<CtrlSpec> {
        PwmStopOnKillW::new(self, 3)
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    #[must_use]
    pub fn generic(&mut self) -> GenericW<CtrlSpec> {
        GenericW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    #[must_use]
    pub fn up_down_mode(&mut self) -> UpDownModeW<CtrlSpec> {
        UpDownModeW::new(self, 16)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn one_shot(&mut self) -> OneShotW<CtrlSpec> {
        OneShotW::new(self, 18)
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn quadrature_mode(&mut self) -> QuadratureModeW<CtrlSpec> {
        QuadratureModeW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlSpec> {
        ModeW::new(self, 24)
    }
}
#[doc = "Counter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
