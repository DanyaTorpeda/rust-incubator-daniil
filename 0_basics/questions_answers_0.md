## What memory model does Rust have? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?

Rust uses a memory management model based on ownership, borrowing, and lifetimes. Memory is freed automatically when the owner goes out of scope, without the need for garbage collection or manual freeing.

Ownership is a system in which each value has a single owner. The owner manages the value's lifetime, can transfer ownership to other variables, and is responsible for releasing resources when the value is destroyed.

Borrowing is a mechanism for temporarily accessing data without transferring ownership. A value can have any number of immutable references or a single mutable reference.

## What runtime does Rust have? Does it use a GC (garbage collector)?

Rust does not have a mandatory runtime, but it provides a minimal runtime through its standard library for program startup, stack management, and threading. For asynchronous programming, Rust relies on external runtimes such as Tokio or async-std to handle task scheduling and execution.

Rust does not use a garbage collector.

## What does static typing mean? What is a benefit of using it?

Static typing means that the type of variables and expressions is known and checked at compile time, before the program is executed. 

A key benefit of static typing is that many errors related to incorrect types are caught at compile time instead of runtime.

## What are generics and parametric polymorphism? Which problems do they solve?

Generics are a language feature that allows writing functions, structs, or traits that work with different types without duplicating code. They let the compiler generate type-specific versions of the same logic while keeping type safety.

Parametric polymorphism is the underlying concept behind generics, where a function or type is written in a way that it can operate uniformly over any type without depending on its specific meaning.

They allow the same code to be reused for different types, reducing code duplication while providing type-safe abstractions and maintaining performance, since the actual types are resolved at compile time.

## Question: What are traits? How are they used? How do they compare to interfaces? What are auto traits and blanket impls? What is a marker trait?

Traits are a language feature that define shared behavior that types can implement. They allow abstraction over behavior and are used to write generic functions that work with any type that satisfies certain requirements.

Compared to interfaces in other languages, traits support features like associated types, blanket implementations, and can be implemented for external types under certain rules.

Auto traits are traits that the compiler can automatically implement for a type if all of its components satisfy certain conditions, such as Send and Sync.

Blanket implementations are trait implementations that apply to all types satisfying a given constraint, allowing behavior to be automatically shared across multiple types without writing separate implementations.

Marker traits are traits without methods that exist only to mark types with certain properties, such as Send, Sync, or Unpin.

## What are static and dynamic dispatch? Which should you use, and when?

Static dispatch means that the compiler determines which function implementation to call at compile time using generics. This is achieved through monomorphization and results in highly optimized code with no runtime overhead.

Dynamic dispatch means that the decision of which implementation to call is made at runtime using trait objects and a vtable. This provides flexibility but introduces a small runtime cost.

Static dispatch should be used when performance is important and types are known at compile time. Dynamic dispatch should be used when different types need to be handled uniformly through a shared interface.

## What is a crate and what is a module in Rust? How do they differ? How are they used?

A crate is the basic unit of compilation in Rust. It can be either a binary crate, which produces an executable and contains a main function, or a library crate, which provides reusable functionality for other crates.

A module is a way to organize code within a crate. Modules help structure code into logical units, control visibility, and create a hierarchical organization.

The main difference is that a crate is a compilation unit, while a module is an organizational unit inside a crate. A crate contains modules, and modules can contain submodules, forming a tree-like structure of the program.

## What are move semantics? What are borrowing rules? What is the benefit of using them?

Move semantics in Rust define that values are moved rather than copied by default, meaning ownership of a value is transferred from one variable to another, and the previous owner becomes invalid.

Borrowing rules allow temporary access to data without transferring ownership. Rust allows either multiple immutable references or a single mutable reference at the same time, but not both, to ensure memory safety.

The benefit of these mechanisms is that Rust can guarantee memory safety and prevent data races at compile time without needing a garbage collector or manual memory management, while still maintaining high performance and predictable behavior.

## What is immutability? What is the benefit of using it?

Immutability means that a value cannot be changed after it is created. In Rust, variables are immutable by default unless explicitly marked as mutable.

The main benefit of immutability is increased safety and predictability, since values cannot be accidentally modified from different parts of the program. It also helps prevent bugs, makes code easier to reason about, and improves safety in concurrent contexts because immutable data can be safely shared between threads.

## What is cloning? What is copying? How do they compare?

Copy means that a value is duplicated implicitly when it is assigned or passed to a function. Types that implement Copy are types whose values can be duplicated by a simple bitwise copy. They are typically small value types and must not implement Drop.

Clone is an explicit operation that creates a full, independent copy of a value, potentially including heap-allocated data. It must be called manually using the .clone() method.

The main difference is that Copy is implicit and inexpensive, while Clone is explicit and may involve deeper or more expensive duplication of data.

## What is RAII? How is it implemented in Rust? What is the benefit of using it?

RAII (Resource Acquisition Is Initialization) is a programming concept where resources such as memory, files, or network connections are tied to the lifetime of an object, and are automatically released when the object goes out of scope.

In Rust, RAII is implemented through the ownership system and the Drop trait. When a value goes out of scope, its destructor (drop) is automatically called, which releases any associated resources.

The benefit of RAII is that it ensures deterministic resource management without requiring manual cleanup or a garbage collector, preventing resource leaks and making programs safer and more predictable.

## What is an iterator? What is a collection? How do they differ? How are they used?

An iterator is an object that provides sequential access to elements of a data source, producing one item at a time through methods like next(). Iterators are often lazy, meaning they only compute or fetch values when requested.

A collection is a data structure that stores and owns multiple values, such as a vector, hash map, or set.

The main difference is that collections store data, while iterators provide a way to traverse or process that data. Iterators are commonly used for loops and functional-style operations like map and filter, allowing efficient and composable data processing without needing to manipulate the collection directly.

## What are macros? Which problems do they solve? What is the difference between declarative and procedural macros?

Macros in Rust are a compile-time feature that generates code before the program is compiled. They are used to reduce boilerplate, generate repetitive code, and extend the language with custom behavior.

Declarative macros (using macro_rules!) work by pattern matching and substituting code based on predefined templates. They are simpler and mainly used for repetitive code patterns.

Procedural macros are more powerful and operate by taking Rust code as input, processing it during compilation, and generating new code. They can analyze and transform the abstract syntax tree, allowing for complex code generation such as derive macros.

The main difference is that declarative macros are pattern-based code substitution, while procedural macros are programmatic code generation during compilation.

## How is code tested in Rust? Where should you put tests and why?

Code in Rust is tested using built-in test support provided by the language. Unit tests are written inside the same file as the code, usually in a special module marked with #[cfg(test)], and they test internal logic of a module.

Integration tests are placed in a separate tests directory and test the public API of a crate as an external user would use it. Additionally, Rust supports documentation tests, where code examples inside documentation comments are automatically executed as tests.

Unit tests are placed next to the implementation because they often need access to private items, while integration tests are kept separate because they verify the public API from the perspective of an external user.

## Why does Rust have &str and String types? How do they differ? When should you use them?

Rust has both String and &str because they serve different purposes. String is an owned, heap-allocated, mutable string type used when you need to create or modify string data. &str is a borrowed string slice, which is a reference to string data and does not own it.

The main difference is ownership: String owns its data, while &str only provides a view into existing string data. This makes &str more lightweight and efficient for reading, while String is used when you need to own or modify the string.

Use String when ownership or mutation is required, and use &str when only read-only access to string data is needed.

## What are lifetimes? Which problems do they solve? Which benefits do they give?

Lifetimes are a mechanism in Rust that describes how long references are valid and how they relate to the data they point to. They ensure that references never outlive the data they refer to.

They solve problems such as dangling references and use-after-free errors by enforcing memory safety at compile time. Without lifetimes, it would be possible to access memory that has already been deallocated.

The benefit of lifetimes is that they provide strong compile-time guarantees about memory safety without introducing runtime overhead or requiring a garbage collector, while allowing safe and efficient use of references.

## Is Rust an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance?

Rust is not a traditional object-oriented programming language because it does not have classes or inheritance. However, it supports many object-oriented principles such as encapsulation, polymorphism, and abstraction through structures, modules, and traits.

Instead of inheritance, Rust uses composition, where types are built by combining other types, and traits to define shared behavior. This allows flexible and safe polymorphism without relying on class hierarchies.

Design principles like SOLID and GRASP can still be applied in Rust, but they are achieved through traits, modules, and the ownership system rather than built-in object-oriented features.