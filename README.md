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

impl Destroy<()> for MyDestroyableItem {
    fn destroy(self) {
        // Do things to destroy item
    }
}

fn main() {
    let destroy_me = MustDestroy::<_, ()>::new(MyDestroyableItem);
 
    // Dropping the item here would cause a panic at runtime
    // drop(destroy_me) 
    
    // However calling destroy will consume the item, and not cause
    // a panic.
    destroy_me.destroy();
}
```