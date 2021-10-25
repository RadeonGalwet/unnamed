# Unnamed Language

## Examples

```js
function sum(a: int32, b: int32) -> int32 = a + b;
function main() {
  let result = sum(1, 2);
  if result == 3 {
    print("result == 3");
  } else {
    print("result != 3");
  }
}
```

## To Do

- [ ] Support to unsigned integers
- [x] REFACTOR COMPILER!!!
- [x] Conditional statements
- [x] Tests for compiler
- [ ] Parser tests
- [ ] Support to void types & functions without return
- [ ] Extern blocks