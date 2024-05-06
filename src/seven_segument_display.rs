use defmt::info;
use embedded_hal::digital::OutputPin;
use rp2040_hal::gpio::{FunctionSio, Pin, PinId, PullDown, SioOutput};

type PinType<PinPos> = Pin<PinPos, FunctionSio<SioOutput>, PullDown>;

enum Position {
    OverLeft = 0,
    OverTop = 1,
    OverRight = 2,
    Center = 3,
    UnderLeft = 4,
    UnderBottom = 5,
    UnderRight = 6,
    #[allow(dead_code)]
    Dot = 7,
}

pub struct SevenSegumentDisplay<
    OL: PinId,
    OT: PinId,
    OR: PinId,
    C: PinId,
    UL: PinId,
    UB: PinId,
    UR: PinId,
    D: PinId,
> {
    over_left: PinType<OL>,
    over_top: PinType<OT>,
    over_right: PinType<OR>,
    center: PinType<C>,
    under_left: PinType<UL>,
    under_bottom: PinType<UB>,
    under_right: PinType<UR>,
    #[allow(dead_code)]
    dot: PinType<D>,
}

impl<OL: PinId, OT: PinId, OR: PinId, C: PinId, UL: PinId, UB: PinId, UR: PinId, D: PinId>
    SevenSegumentDisplay<OL, OT, OR, C, UL, UB, UR, D>
{
    pub fn new(
        over_left: PinType<OL>,
        over_top: PinType<OT>,
        over_right: PinType<OR>,
        center: PinType<C>,
        under_left: PinType<UL>,
        under_bottom: PinType<UB>,
        under_right: PinType<UR>,
        dot: PinType<D>,
    ) -> Self {
        Self {
            over_left,
            over_top,
            over_right,
            center,
            under_left,
            under_bottom,
            under_right,
            dot,
        }
    }

    pub fn set_number(&mut self, n: &u8) {
        match *n {
            0 => self.render([true, true, true, false, true, true, true, false]),
            1 => self.render([false, false, true, false, false, false, true, false]),
            2 => self.render([false, true, true, true, true, true, false, false]),
            3 => self.render([false, true, true, true, false, true, true, false]),
            4 => self.render([true, false, true, true, false, false, true, false]),
            5 => self.render([true, true, false, true, false, true, true, false]),
            6 => self.render([true, true, false, true, true, true, true, false]),
            7 => self.render([false, true, true, false, false, false, true, false]),
            8 => self.render([true, true, true, true, true, true, true, false]),
            9 => self.render([true, true, true, true, false, true, true, false]),
            _ => info!("Not number"),
        }
    }

    fn render(&mut self, pattern: [bool; 8]) {
        switch(&mut self.over_left, pattern[Position::OverLeft as usize]);
        switch(&mut self.over_top, pattern[Position::OverTop as usize]);
        switch(&mut self.over_right, pattern[Position::OverRight as usize]);
        switch(&mut self.center, pattern[Position::Center as usize]);
        switch(&mut self.under_left, pattern[Position::UnderLeft as usize]);
        switch(
            &mut self.under_bottom,
            pattern[Position::UnderBottom as usize],
        );
        switch(
            &mut self.under_right,
            pattern[Position::UnderRight as usize],
        );
    }
}

fn switch<P: PinId>(target: &mut PinType<P>, status: bool) {
    if status {
        target.set_high().unwrap();
    } else {
        target.set_low().unwrap();
    };
}
