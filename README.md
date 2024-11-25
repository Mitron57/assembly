# Assembler/Interpreter

# Usage
- Usage: assembly --inp `<INPUT>` --output-asm `<OUTPUT_ASM>` --output-int `<OUTPUT_INT>` --log `<LOG>` --capacity `<CAPACITY>`

- Options:
  - --inp `<INPUT>`
  - -a, --output-asm `<OUTPUT_ASM>`
  - -i, --output-int `<OUTPUT_INT>`  
  - -l, --log `<LOG>`
  - -c, --capacity `<CAPACITY>`      
  - -h, --help                     Print help

# Requirements
 - Rust roolchain 


# Example
Input:
```asm
loadc 63
store 10
load 10
store 20
popcnt 604, 20
```

Log:
```yaml
loadc:
  command_num: 118
  lhs: 63
store:
  command_num: 238
  lhs: 20
popcnt:
  command_num: 132
  lhs: 604
  rhs: 20
load:
  command_num: 231
  lhs: 10
```

Output:
```yaml
604:
  order: bitvec::order::Lsb0
  head:
    width: 8
    index: 0
  bits: 14
  data:
  - 6
  - 0
20:
  order: bitvec::order::Lsb0
  head:
    width: 8
    index: 0
  bits: 14
  data:
  - 63
  - 0
10:
  order: bitvec::order::Lsb0
  head:
    width: 8
    index: 0
  bits: 14
  data:
  - 63
  - 0
```
