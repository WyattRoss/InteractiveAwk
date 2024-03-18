use cursive::{view::Resizable, views::{DummyView, LinearLayout, Panel}};

mod invoker;
mod provider;
mod util;

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(
        LinearLayout::vertical()
            .child(
                Panel::new(DummyView)
                    .title("Awk Code")
                    .fixed_width(80)
                    .fixed_height(3)
            )
            .full_screen()
    );
    siv.run();
}
