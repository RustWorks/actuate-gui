use actuate::{use_state, virtual_dom, Scope, View, ViewBuilder};

struct A;

impl View for A {
    fn body(&self, cx: &Scope) -> impl ViewBuilder {
        dbg!("A!");
    }
}

struct Counter {
    initial: i32,
}

impl View for Counter {
    fn body(&self, cx: &Scope) -> impl ViewBuilder {
        let (count, set_count) = use_state(cx, || self.initial);

        set_count.set(count + 1);

        dbg!(count);

        (*count == 2).then_some(A)
    }
}

struct App;

impl View for App {
    fn body(&self, _cx: &Scope) -> impl ViewBuilder {
        Counter { initial: 2 }
    }
}

#[tokio::main]
async fn main() {
    let mut vdom = virtual_dom(App);

    vdom.run().await;
    vdom.run().await;

    dbg!(vdom);
}
