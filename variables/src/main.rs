fn main() {
    // immutable variable vs constants:
    // - Constants are always immutable
    // - Declaration: const keyword + type of the value
    // - Can be declared in any scope: locally, globally
    // - Can not be set as result of a function call or any valued computed at runtime
    // constants
    let x = 5;
    // Shadowing vs mut:
    // - Shadowing requires let keyword to reassign new value
    // - You can change the type  of the value when use Shadowing. Moreover, it spare us from having come up with different variable names. Ex: x_str, x_int
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
}
