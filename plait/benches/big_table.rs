use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use plait::{ToHtml, html};

fn big_table_plait_simple(b: &mut Bencher<'_>, size: &usize) {
    let table: Vec<Vec<usize>> = (0..*size).map(|_| (0..*size).collect()).collect();

    let big_table = html! {
        table {
            for r1 in table.iter() {
                tr {
                    for r2 in r1.iter() {
                        td {
                            (r2)
                        }
                    }
                }
            }
        }
    };

    b.iter(|| big_table.to_html());
}

plait::component! {
    pub fn BigTablePlait(table: &Vec<Vec<usize>>) {
        table {
            for r1 in table.iter() {
                tr {
                    for r2 in r1.iter() {
                        td {
                            (r2)
                        }
                    }
                }
            }
        }
    }
}

fn big_table_plait_component(b: &mut Bencher<'_>, size: &usize) {
    let table: Vec<Vec<usize>> = (0..*size).map(|_| (0..*size).collect()).collect();

    let big_table = html! {
        @BigTablePlait(table: &table) {}
    };

    b.iter(|| big_table.to_html());
}

markup::define! {
    BigTableMarkup(table: Vec<Vec<usize>>) {
        table {
            @for r1 in table {
                tr {
                    @for r2 in r1 {
                        td { @r2 }
                    }
                }
            }
        }
    }
}

fn big_table_markup(b: &mut Bencher<'_>, size: &usize) {
    let table: Vec<Vec<usize>> = (0..*size).map(|_| (0..*size).collect()).collect();
    let big_table = BigTableMarkup { table };

    b.iter(|| big_table.to_string());
}

fn bench_big_table_plait(c: &mut Criterion) {
    let input = 100;

    let mut group = c.benchmark_group("Big table");

    group.bench_with_input("Plait simple", &input, big_table_plait_simple);
    group.bench_with_input("Plait component", &input, big_table_plait_component);
    group.bench_with_input("Markup", &input, big_table_markup);

    group.finish();
}

criterion_group!(benches, bench_big_table_plait);
criterion_main!(benches);
