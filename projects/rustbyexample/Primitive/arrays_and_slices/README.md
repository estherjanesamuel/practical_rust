# Arrays and Slices

An array is a collection of objects of the same type `T`, stored in contagious memory. Arrays are created using brackets [], and their length , which is known at compile time, is part of their type signature [T; length].

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object, the first word is a pointer to data, and the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture e.g 64 bits on an x86-64. Slices can be used to borrow section of array, and have the type signature &[T].
