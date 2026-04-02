// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// -- I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software: impl Licensed, software_two:impl Licensed) -> bool{
// fn compare_license_types<T:Licensed>(software: T, software_two:T) -> bool{
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}



// 问: 关于 compare_license_types  参数定义中不可以使用 <T:Licensed>(software: T, software_two:T)
// 答:


// 您提出了一个很好的问题，这涉及到 Rust 中泛型和 trait bounds 的一些细微差别。让我们逐一分析这两种情况：

// 1. 第一个例子（不能使用泛型）：

// ```rust
// fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool
// ```

// 这里不能使用泛型 `<T: Licensed>` 的原因是：

// - 使用 `impl Licensed` 允许两个参数是不同的类型，只要它们都实现了 `Licensed` trait。
// - 如果改为 `<T: Licensed>`，就要求两个参数必须是相同的类型 T。

// 在这个场景中，我们希望比较两个可能不同类型的软件的许可信息。使用 `impl Licensed` 提供了更大的灵活性，允许比较 `SomeSoftware` 和 `OtherSoftware` 的许可信息。

// 2. 第二个例子（可以使用泛型）：

// ```rust
// fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool
// ```

// 这里可以使用泛型，因为：

// - 函数只有一个参数，所以使用泛型 T 不会引起类型不匹配的问题。
// - 泛型 T 有多个 trait bounds（`SomeTrait + OtherTrait`），这在语法上更简洁。

// 两种写法的主要区别：

// 1. 灵活性：
//    - `impl Trait` 语法允许每个参数是不同的具体类型。
//    - 泛型 `<T: Trait>` 要求所有使用 T 的地方都是同一个具体类型。

// 2. 代码可读性：
//    - 对于简单情况，`impl Trait` 可能更易读。
//    - 对于复杂的 trait bounds，泛型语法可能更清晰。

// 3. 返回值：
//    - `impl Trait` 可以用在返回值位置，而泛型不行。

// 总结：
// - 第一个例子中，使用 `impl Licensed` 是为了允许比较两个不同类型的软件。
// - 第二个例子中，使用泛型 `<T: SomeTrait + OtherTrait>` 是可行的，因为只有一个参数，并且它简化了多个 trait bounds 的表达。

// 选择使用 `impl Trait` 还是泛型通常取决于具体的使用场景和需求。在需要更大灵活性的情况下，`impl Trait` 可能更合适；而在需要在函数体内部使用类型 T 或者有多个相同类型参数的情况下，泛型可能是更好的选择。

