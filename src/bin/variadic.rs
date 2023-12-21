// Step 1: Define the trait
pub trait MyFunction<T>: Send + Sync + 'static {
    fn call(&self, args: T);
}

// Step 2: Define the macro
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
// Step 3: all_the_tuples!() macro is to generate the 0-3 arguments with any type for the called macro, in this case impl_my_function!
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