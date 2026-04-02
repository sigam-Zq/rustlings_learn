// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// -- I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self{
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}


//   提问 ： trait 中的参数  self  不带有 mut 关键字

// 实现的时候带有 mut 关键字，这里 可以接受么 ，为什么

// 答：
// 您提出了一个很好的问题。这种情况是完全可以接受的，让我来解释一下原因：

// 1. trait 定义中的 `self`:
//    在 trait 定义中，`self` 参数不带 `mut` 关键字。这是因为 trait 定义描述的是一个通用的接口，它不指定方法是否需要修改 `self`。

// 2. trait 实现中的 `mut self`:
//    在具体实现中，我们可以根据需要选择是否使用 `mut self`。这里使用 `mut self` 是因为方法需要修改 `self`（通过 `push` 方法添加新元素）。

// 3. 为什么这是允许的：
//    - Rust 允许在实现 trait 时，根据具体类型的需求，增加但不能减少对 `self` 的限制。
//    - 将 `self` 变为 `mut self` 是增加限制，因为它要求调用者必须拥有可变的所有权。
//    - 这种灵活性允许实现者根据具体情况选择最合适的方式来实现 trait 方法。

// 4. 重要的一点：
//    虽然实现使用了 `mut self`，但调用这个方法时仍然可以使用不可变的 `Vec<String>`。Rust 会在方法调用时自动将不可变引用转换为可变的。

// 例如：

// ```rust
// let v = vec![String::from("Foo")];
// let v2 = v.append_bar(); // 这里 v 是不可变的，但方法调用仍然有效
// ```

// 总结：trait 定义中使用 `self`，而实现中使用 `mut self` 是完全可以接受的。这种做法提供了更大的灵活性，允许实现者根据具体需求来决定是否需要修改 `self`。



// ------------------------------------------------------------------------------------------------------
// 提问： 这里的 小写 self 和首字母大写的 Self 有什么关联么，这里小写和大写是有什么约定在么

// 答：

// 这是一个很好的问题！在 Rust 中，`self` 和 `Self` 确实有不同的含义和用途。让我解释一下它们之间的区别和关联：

// 1. 小写 `self`:
//    - 表示当前实例的引用。
//    - 在方法定义中用作第一个参数。
//    - 可以是 `self`、`&self`、或 `&mut self`，分别表示值、不可变引用、和可变引用。

// 2. 大写 `Self`:
//    - 是一个类型，表示实现 trait 或方法的类型本身。
//    - 在 trait 定义和实现中用作返回值类型或参数类型。
//    - 是一个隐式的关联类型，代表 "实现这个 trait 的具体类型"。

// 3. 关联和用法:
//    - 在方法签名中，`self` 的类型实际上就是 `Self`。
//    - `Self` 允许我们在不知道具体类型的情况下引用实现 trait 的类型。

// 4. 约定:
//    - 使用 `self` 作为方法的第一个参数是一种约定，表示这是一个实例方法。
//    - 使用 `Self` 作为类型名是一种约定，用于指代当前类型，增加代码的灵活性和可重用性。

// 让我们用您的例子来说明：

// ```rust
// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for Vec<String> {
//     fn append_bar(mut self) -> Self {
//         self.push(String::from("Bar"));
//         self
//     }
// }
// ```

// 在这个例子中：
// - `self` 在 trait 定义和实现中都表示方法的接收者（即 `Vec<String>` 的实例）。
// - `Self` 在返回类型中使用，表示 "实现这个 trait 的类型"。对于 `Vec<String>` 的实现来说，`Self` 就等同于 `Vec<String>`。

// 使用 `Self` 作为返回类型使得 trait 更加通用，因为它可以适用于任何实现该 trait 的类型，而不需要明确指定具体类型。

// 总之，`self` 和 `Self` 的使用是 Rust 中的重要约定，它们共同工作以提供类型安全和代码重用的机制。