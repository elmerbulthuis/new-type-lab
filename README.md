# new-type lab

Playing with the new-type idiom for fun and profit!

## why

Custom deserialization is easy to implement on a new-type type. One use case would be to use validation in deserialization.

## why not

The ergonomics of new-type are not great. Every property on a struct would be a new-type so this is a lot of less ergonomic code.

## thoughts

We try to make the ergonomics a little bit better by implementing some traits. The From (and Into) trait seems to be a no brainer, but the Deref trait is more interesting. According to the [docs](https://doc.rust-lang.org/std/ops/trait.Deref.html), `Deref should only be implemented for smart pointers`. Now the new-type types are not smart pointers so that seems like a no-go.

But then again there are also (non official rustlang docs) that are not as strict:

- https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html

So the deref is sometimes misused to unwrap new-type types. One way to accept this would be to consider a new-type the same as a smart pointer on some level.

Another approach would be to use AsRef. And here it gets interesting, the [docs](https://doc.rust-lang.org/std/convert/trait.AsRef.html) say `However, AsRef::as_ref should not be used for the sole purpose of dereferencing; instead ‘Deref coercion’ can be used`. But are we not using AsRef then for the sole purpose of dereferencing? Well you could say that we are dereferencing a new-type, but can you even dereference a new-type? I guess dereferencing is a thing you do with pointers.

So once again the question is if we consider a new-type a pointer. If we do, we can use Deref, if we don't we can use AsRef. Of course a new-type is not a pointer, but if we consider is as such we can bend reality a little bit.

A reason to do this would be ergonomics, the difference is basically this:

```rust
for item in list.as_ref().iter() {
    let id = *item.id.as_ref();
    let name = item.name.as_ref();

    assert_eq!(id, 1);
    assert_eq!(name, "abc");
}
```

vs

```rust
for item in list.iter() {
    let id = *item.id;
    let name: &str = &item.name;

    assert_eq!(id, 1);
    assert_eq!(name, "abc");
}

```

Having `as_ref`s scattered in your code is not really what you want. But then again it's not evil.

But why not make the wrapped value public? Maybe implement `Into` and `From`, but that's it then. We have a bunch of `.0` scattered around the code, but this might be better than having `.as_ref()` everywhere.

Looks like this:

```rust
for item in list.0.iter() {
    let id = item.id.0;
    let name = &item.name.0;

    assert_eq!(id, 1);
    assert_eq!(name, "abc");
}
```
