pub fn boxing_unboxing() {
    let a = 7;
    let a_box: Box<i32>;
    let mut a_ref: &i32 = &a;
    print!("{} {};", a, *a_ref);
    a_box = Box::new(a + 2);
    a_ref = &*a_box;
    print!(" {} {} {}", a, *a_ref, *a_box);
}

// This will print: 7 7; 7 9 9. Let’s examine it.
// The stack will contain three objects: the number 7, represented by the a variable;
// and two pointers, represented by the a_box and a_ref variables. The pointer declared
// in the second line, a_box, is initialized only at the fifth line, while the other variables are
// initialized in the same statements in which they are declared.
// Both pointer variables are type annotated, but such annotations are optional and
// could be removed, because their types can be inferred from their usage. However, they
// show that a_box is a smart pointer, while a_ref is a dumb pointer, meaning that a_box
// takes care of allocating and deallocating its referred object, while a_ref does not take
// care of either allocation or deallocation of its referred object.
// In the two print macro invocations, the three asterisks are optional and could be
// removed.
// In the fifth and sixth lines, the two pointers are assigned a value. But for a_box, it’s an
// initialization, so the variable does not have to be mutable; however, a_ref had already
// been initialized, and so, to be reassigned it had to be declared with the mut clause.
// The third line just set the a_ref variable value as the address of the a variable. But
// the fifth line does something more complex. It allocates an i32 object in the heap,
// initializes that object using the value of the expression a + 2, which is the value 9, and
// then it sets the a_box variable value as the address of that object.
// In the sixth line, the value of a_box (not the value referred to by a_box), which is a
// pointer, is copied to the variable a_ref; in other words, the dumb pointer is made to
// point to the same object pointed to by the smart pointer. This is confirmed by the last
// print statement. However, this assignment cannot be simply a_ref = a_box;, because
// the two variables have different types, and also the statement a_ref = a_box as &i32;
// would be illegal. Instead, the trick of dereferencing using *, and then referencing using &
// allows us to convert a Box into a reference. Better said, it allows us to take the address of
// the object referenced to by the box.
// Notice that the inverted operation would be illegal: a_box = &*a_ref;. Actually, the
// expression &*a_ref is still of type &i32, and so it cannot be assigned to a variable of type
// Box<i32>.
// Finally, at the end of the program, first a_ref exits from its scope, doing nothing;
// then a_box exists from its scope, deallocating its referenced object from the heap; then a
// exits from its scope, doing nothing; and at last the three objects are deallocated from the
// stack.

// This program is similar:
pub fn boxing_unboxing2() {
    let a = 7;
    let mut a_box: Box<i32>;
    let a_ref: &i32 = &a;
    print!("{} {};", a, a_ref);
    a_box = Box::new(a + 2);
    print!(" {} {} {};", a, a_ref, a_box);
    a_box = Box::new(*a_ref);
    print!(" {} {} {}", a, a_ref, a_box);
}

// It will print 7 7; 7 7 9; 7 7 7.
// Here it is a_box to be mutable, while a_ref is immutable.