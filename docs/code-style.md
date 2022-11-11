---
title: Code Style
---

All rs source file must write in english and can not contains chinese word

Please make sure your code pass fmt/clippy checking, and pass our extra clippy lints.

Here is some extras lints you need to pass

## use guard statement reduce if/match block nested

Bad example:

```rust
if flag1 {
    if flag2 {
        match xxx_opt {
            Some(xxx) => {
                // do some thing
            },
            None
        }
    } else {
        return None;
    }
} else {
    return None;
}
```

Good example:

```rust
if !flag {
    return None;
}
if !flag2 {
    return None;
}
let xxx = match xxx_opt {
    Some(xxx) => {
        xxx
    },
    None
}
// do some thing
```

## use Future join await multi Future

Bad example:

```rust
for cell_update_item in cells {
    app_context
        .redis_cache
        .partial_update_cell(&path, cell_update_item, project_id)
        .await?;
}
```

Good example:

```rust
let mut futs = Vec::new();
for cell_update_item in cells {
    futs.push(
        app_context
            .redis_cache
            .partial_update_cell(&path, cell_update_item, project_id),
    );
}
futures::future::try_join_all(futs).await?;
```
