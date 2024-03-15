use cursive::views::LinearLayout;

mod invoker;
mod provider;

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(LinearLayout::horizontal());
    siv.run();
}
