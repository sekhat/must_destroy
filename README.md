# must_destroy

Must destroy is used to create a paramterized destructor for a type
that must be explicitly called.

`MustDestroy<T, Args>` acts as a guard for a wrapped type that implements the `Destroy`
trait, that causes a `panic` if the guard is dropped.

However, calling destroy upon the guard, will call destroy on wrapped child, and will
be consumed safely.

```rust
use must_destroy::{MustDestroy, Destroy};
    struct MyDestroyableItem;

    impl Destroy<(&'_ str, i32)> for MyDestroyableItem {
        fn destroy(self, args: (&str, i32)) {
            
            // Do things to destroy item...

            // Just to show our arguments got through fine
            assert_eq!("Test String", args.0);
            assert_eq!(12, args.1);
        }
    }

    fn main() {
        let destroy_me = MustDestroy::new(MyDestroyableItem);

        // Dropping the item here would cause a panic at runtime
        // drop(destroy_me)

        // However calling destroy will consume the item, and not cause
        // a panic.
        
        // We currently have to pass the arguments as a tuple.
        //
        // I'd like to be able to hide the need to do this though.
        destroy_me.destroy(("Test String", 12));
    }
```