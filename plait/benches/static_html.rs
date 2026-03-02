use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use plait::{ToHtml, html};

fn static_html_plait(b: &mut Bencher<'_>) {
    let static_html = html! {
        div {
            "Hello"
        }
    };

    b.iter(|| static_html.to_html());
}

markup::define! {
    StaticHtml {
        div {
            "Hello"
        }
    }
}

fn static_html_markup(b: &mut Bencher<'_>) {
    let hello = StaticHtml {};

    b.iter(|| hello.to_string());
}

fn bench_static_html_plait(c: &mut Criterion) {
    let mut group = c.benchmark_group("Static html");

    group.bench_function("Plait", static_html_plait);
    group.bench_function("Markup", static_html_markup);
    group.finish();
}

criterion_group!(benches, bench_static_html_plait);
criterion_main!(benches);
