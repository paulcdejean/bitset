Aims to implement roughly the interface of std::HashSet.

Although note that set items are inserted and extracted by value and not by reference. 

| Function | Status | Notes |
| ------------------------------ | ------------------------ | ----------------------- |
| capacity | ✅ | Returns the number of bits the bitset can hold, which is fixed. |
| clear | TODO |
| contains | TODO |
| difference | TODO |
| drain | TODO |
| entry | TODO |
| extract_if | TODO |
| get | TODO |
| get_or_insert | TODO |
| get_or_insert_with | TODO |
| hasher | ❌ | Hasher related functions won't be implemented. |
| insert | ✅ 
| intersection | TODO
| is_disjoint | TODO
| is_empty | TODO
| is_subset | TODO
| is_superset | TODO
| iter | TODO
| len | ✅
| new | ✅ | The new bitset is initalized to be empty.
| remove | TODO
| replace | TODO
| reserve | ❌ | Won't be implemented as bitsets can't change size. |
| retain | TODO
| shrink_to | ❌ | BitSet capacity can't change.
| shrink_to_fit | ❌ | BitSet capacity can't change.
| symmetric_difference | TODO
| take | TODO
| try_reserve | ❌ | BitSet capacity can't change.
| union | TODO
| with_capacity | ❌ | BitSet capacity is fixed.
| with_capacity_and_hasher | ❌
| with_hasher | ❌ | Hasher related functions won't be implemented. |

| Trait | Status |
| ------------------------------ | ------------------------ |
BitAnd | TODO
BitOr | TODO
BitXor | TODO
Clone | ✅
Debug | TODO
Default | ✅
Eq | ✅
Extend<&'a T> | TODO
Extend<T> | TODO
From<[T; N]> | TODO
FromIterator<T> | TODO
IntoIterator | TODO
PartialEq | ✅
Sub | TODO
