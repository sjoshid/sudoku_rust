use crate::{BASE, RECOMPUTE_SELECTOR, SIZE, SIZE2};
use druid::{widget::*, *};
use radix_fmt::radix; // TODO capitalize

#[derive(Clone, Data)]
pub struct Cell {
    pub value: Option<u8>,
    pub possibilities: [bool; SIZE2],
    user_removed: [bool; SIZE2],
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: None,
            possibilities: [true; SIZE2],
            user_removed: [false; SIZE2],
        }
    }
}

pub struct GridSpace {
    root: WidgetId,
    textbox: TextBox,
    display: Container<Cell>,
}

impl GridSpace {
    pub fn new(root: WidgetId) -> Self {
        Self {
            root,
            textbox: TextBox::new(),

            display: Either::new(
                |c: &Cell, _| c.value.is_some(),
                Self::make_value_label(),
                Self::make_possibility_grid(),
            )
            .center()
            .background(Color::WHITE)
            .border(Color::BLACK, 0.5),
        }
    }

    fn make_value_label() -> impl Widget<Cell> {
        Label::dynamic(|c: &Cell, _| {
            c.value
                .map(|n| radix(n, BASE).to_string())
                .unwrap_or_default()
        })
        .with_text_size(48.0) // TODO: look into flexing text size
        .with_text_color(Color::BLACK)
    }

    // TODO mess with alignments for better look?
    fn make_possibility_grid() -> impl Widget<Cell> {
        let mut column = Flex::column();
        for y in 0..SIZE {
            let mut row = Flex::row();
            for x in 0..SIZE {
                row.add_flex_child(
                    Label::dynamic(move |c: &Cell, _| {
                        if c.possibilities[y * SIZE + x] {
                            radix(y * SIZE + x + 1, BASE).to_string()
                        } else {
                            String::new()
                        }
                    })
                    .with_text_size(13.0) // TODO: look into flexing text size
                    .with_text_color(Color::BLACK)
                    .center(),
                    1.0,
                );
            }
            column.add_flex_child(row, 1.0);
        }

        column
    }
}

impl Widget<Cell> for GridSpace {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Cell, env: &Env) {
        // TODO handle arrow keys and tab
        let new_val = match event {
            Event::KeyDown(KeyEvent { key, .. }) => match key {
                KbKey::Character(c) => c
                    .chars()
                    .last()
                    .and_then(|c| c.to_digit(BASE as u32))
                    .map(|n| n as u8)
                    .filter(|&n| n != 0)
                    .or(data.value),

                KbKey::Backspace | KbKey::Delete => None, // TODO add others?

                _ => data.value,
            },
            _ => {
                // Pretend to be a textbox to get automatic handling of focus, etc.
                self.textbox.event(ctx, event, &mut String::new(), env);
                data.value
            }
        };

        if new_val != data.value {
            data.value = new_val;
            ctx.submit_command(RECOMPUTE_SELECTOR.with(()), self.root);
        }

        self.display.event(ctx, event, data, env); // TODO needed?
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Cell, env: &Env) {
        self.display.lifecycle(ctx, event, &data, env);

        if let LifeCycle::FocusChanged(i_focused) = event {
            if *i_focused {
                self.display.set_background(Color::rgb(0.6, 0.8, 1.0));
            } else {
                self.display.set_background(Color::WHITE);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Cell, data: &Cell, env: &Env) {
        self.display.update(ctx, &old_data, &data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Cell, env: &Env) -> Size {
        self.display.layout(ctx, bc, &data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Cell, env: &Env) {
        self.display.paint(ctx, &data, env);
    }
}