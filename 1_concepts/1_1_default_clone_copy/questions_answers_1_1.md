## What purpose does the Default trait serve in Rust?

The Default trait provides a way to create a type’s “default" value. It is used to allow easy initialization of values when full specification of fields is not necessary. Many primitive types have natural default values, such as 0 for numbers or false for booleans, and complex types like structs or enums can derive Default to automatically construct values where all fields are filled with their own defaults.

## What is #[derive(Default)] from std capable of? What does it wrong? Which are alternatives?

#[derive(Default)] automatically generates an implementation of the Default trait by assigning default values to all fields of a struct. This is useful for simple initialization, but it is limited because it only uses the default values of each field’s type and does not allow expressing meaningful or domain-specific initial states. As a result, it can produce invalid or non-representative values for complex types.

Alternatives include implementing Default manually to define meaningful defaults, using constructor functions like new() to enforce invariants.

## What does Clone mean semantically?

Clone semantically means creating an explicit duplicate of a value that is independent from the original. The cloned value represents the same data, but is a separate object that can be modified or dropped without affecting the original. Unlike implicit copying, cloning must be explicitly requested using the clone() method, which makes the cost and intention visible in the code.

## What does Copy mean semantically? How is it connected with Clone? Which limitations does it have and why?

Copy means that a type can be duplicated implicitly by simply copying its bits, resulting in two independent values with identical data. It is only allowed for types where this duplication is always safe, such as primitive types or shared references. Types that manage resources or own heap memory cannot implement Copy.

Copy is closely related to Clone: every Copy type must also implement Clone, and for Copy types, cloning is equivalent to implicit copying. The key difference is that Copy happens implicitly on assignment, while Clone must be explicitly invoked.