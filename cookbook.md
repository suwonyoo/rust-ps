# String과 &str

## 1. `&str` → `String` 변환

```rust
let s: &str = "hello";
let string_from_str = s.to_string(); // 방법 1
let string_from_str = String::from(s); // 방법 2
let string_from_str = s.into(); // 방법 3 (Into 사용)
```

## 2. `String` → `&str` 변환

```rust
let string_data = String::from("hello");
let str_ref: &str = &string_data; // 방법 1 (참조 사용)
let str_ref: &str = string_data.as_str(); // 방법 2 (as_str() 사용)
```

---
# for문

``.iter`` : 벡터의 요소를 참조 형태로 가져옴 

``enumerate()`` : ``(index, &value)`` 형태의 튜플을 생성, 인덱스를 자동으로 추가가

```rust
    let nums = vec![10, 20, 30, 40];

    // nums.iter().eumerate()로 나온 (index, &value)에서, val1은 value를 자동으로 역참조한다.

    for (i, &val1) in nums.iter().enumerate() {
        println!("Index: {}, Value: {}", i, val1);
    }
    for 

```

`` return vec![j as i32, i as i32]`` 이런 형식으로 값을 반환함

`` let Some(&j) = map.get(&to_check_num)``



```
let str_original_number : String = original_number.to_string();
let reverse : String = str_original_number
.chars()
.rev()
.collect();
```