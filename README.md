Aims to implement roughly the interface of std::HashSet.

Although note that set items are inserted and extracted by value and not by reference. 

| Function | Status | Notes |
| ------------------------------ | ------------------------ | ----------------------- |
| capacity | ✅ | Returns the number of bits the bitset can hold, which is fixed.
| clear | ✅ |
| contains | ✅ |
| difference | ✅ | Rather than returning a lazy iterator, simply returns a new bitset.
| drain | TODO |
| entry | ❌ | This is nightly only experimental for HashSet, so won't be implemented at this time.
| extract_if | ❌ | This is nightly only experimental for HashSet, so won't be implemented at this time.
| get | ✅ |
| get_or_insert | ❌ | This is nightly only experimental for HashSet, so won't be implemented at this time.
| get_or_insert_with | ❌ | This is nightly only experimental for HashSet, so won't be implemented at this time.
| hasher | ❌ | Hasher related functions won't be implemented.
| insert | ✅ 
| intersection | ✅
| is_disjoint | ✅
| is_empty | ✅
| is_subset | ✅ | I will need my logic double checked on this one.
| is_superset | ✅ | I will need my logic double checked on this one.
| iter | TODO
| len | ✅
| new | ✅ | The new bitset is initalized to be empty.
| remove | ✅
| replace | TODO | This is slightly nonsensical as a function.
| reserve | ❌ | BitSet capacity is fixed
| retain | TODO
| shrink_to | ❌ | BitSet capacity is fixed
| shrink_to_fit | ❌ | BitSet capacity is fixed
| symmetric_difference | TODO
| take | TODO
| try_reserve | ❌ | BitSet capacity is fixed
| union | TODO
| with_capacity | ❌ | BitSet capacity is fixed.
| with_capacity_and_hasher | ❌
| with_hasher | ❌ | Hasher related functions won't be implemented.

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
