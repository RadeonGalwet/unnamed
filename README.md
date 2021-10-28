# Unnamed Language

## Examples

```js
function sum(a: int32, b: int32) -> int64 = (a + b) -> int64;
function main() {
  let result = sum(1, 2);
  print(result);
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