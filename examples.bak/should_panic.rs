extern crate laboratory;

fn main() {}

#[allow(unused)]
fn panic_at_the_disco(should_panic: bool) {
    if should_panic {
        panic!("at the disco");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::test::*;

    #[test]
    fn should_panic() {
        describe("panic_at_the_disco()")
            .specs(vec![
                it("should panic when passed true", |_| {
                    should_panic!(panic_at_the_disco, || {
                        panic_at_the_disco(true);
                    })
                }),
                it("should not panic when passed false", |_| {
                    should_not_panic!(panic_at_the_disco, || {
                        panic_at_the_disco(false);
                    })
                }),
            ])
            .run();
    }
}
