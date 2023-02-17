---
title: Code Style
---

<div align="right">

  [English](code-style.md) | [简体中文](code-style_zh.md)

</div>

所有包括rs后缀名在内的Rust代码文件内不能出现中文，必须用英文。

请确保你提交的代码能够通过rustfmt 和 clippy的检测，并符合clippy里的lint级别要求。


以下是你需要通过的lint:

## 使用guard语句减少嵌套的if/match块

糟糕的例子:

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

优秀的例子:

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

## 通过使用.await来执行Future

糟糕的例子:

```rust
for cell_update_item in cells {
    app_context
        .redis_cache
        .partial_update_cell(&path, cell_update_item, project_id)
        .await?;
}
```

优秀的例子:

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
