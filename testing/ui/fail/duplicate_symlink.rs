fn main() {}

dotinstall::installer! {
    symlinks {
        "foo" => "bar",
        "foo" => "baz",
    };
}
