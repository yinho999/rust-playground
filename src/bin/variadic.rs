// Step 1: Define the trait
pub trait MyFunction<T>: Send + Sync + 'static {
    fn call(&self, args: T);
}

// Step 2: Define the macro
///
/// For example: For 3 arguments which is `$name!([T1, T2, T3])` in [`all_the_tuples!`], the [`impl_my_function!`] macro will generate the following code:
/// ``` rust
/// impl<F, T1, T2, T3> MyFunction<(T1, T2, T3)> for F
///    where
///         F: Fn(T1, T2, T3) + Send + Sync + Clone + 'static,
///         T1: Send,
///         T2: Send,
///         T3: Send,
/// {
///    fn call(&self, args: (T1, T2, T3)) {
///       let (t1, t2, t3) = args;
///      (self.clone())(t1, t2, t3);
///   }
/// }
/// ```
///

macro_rules! impl_my_function {
    ([$($ty:ident),*]) => {
        impl<F, $($ty,)*> MyFunction<($($ty,)*)> for F
        where
            F: Fn($($ty,)*) + Send + Sync + Clone + 'static,
            $( $ty: Send, )*
        {
            fn call(&self, args: ($($ty,)*)) {
                let ($($ty,)*) = args;
                (self.clone())($($ty,)*);
            }
        }
    };
}
/// The purpose is: [`all_the_tuples`] macro is used to generate implementations of the [`MyFunction`] trait for functions with different numbers of parameters. It does this by invoking the [`impl_my_function`] macro with different numbers of generic type parameters.
///
/// In Short: [`all_the_tuples!()`] macro is to generate the 0-3 arguments with any type for the called macro, in this case [`impl_my_function!`] macro.
///
/// For example
/// ``` rust
/// all_the_tuples!(impl_my_function)
/// ```
/// will generate the following code:
/// ``` rust
/// macro_rules! all_the_tuples {
///    (impl_my_function) => {
///          impl_my_function!([]);
///         impl_my_function!([T1]);
///         impl_my_function!([T1, T2]);
///         impl_my_function!([T1, T2, T3]);
///     };
///  }
/// ```
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!([]);
        $name!([T1]);
        $name!([T1, T2]);
        $name!([T1, T2, T3]);
        // ... continue for as many arguments as you want to support
    };
}
all_the_tuples!(impl_my_function);

// Define a function that matches the trait bounds
fn my_func(x: i32, y: i32) {
    println!("x: {}, y: {}", x, y);
}

#[tokio::main]
async fn main() {
    // Create a boxed MyFunction
    let my_function: Box<dyn MyFunction<(i32, i32)>> = Box::new(my_func);

    // Call the function through the trait
    my_function.call((5, 10));
}