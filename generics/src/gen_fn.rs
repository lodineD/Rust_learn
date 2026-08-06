// 泛型函数：函数名后面有 <T> 才是泛型函数
// 仅仅使用带泛型参数的结构体不会使函数本身成为泛型函数

struct A;
struct S(A);       // 具体类型
struct SGen<T>(T); // 泛型结构体

// 以下函数都获取传入变量的所有权，并立即离开作用域

// ❌ 不是泛型，参数是具体类型 S
fn reg_fn(_s: S) {}

// ❌ 不是泛型，A 是具体类型，不是函数自身的泛型参数
fn gen_spec_t(_s: SGen<A>) {}

// ❌ 不是泛型，i32 是具体类型
fn gen_spec_i32(_s: SGen<i32>) {}

// ✅ 是泛型！函数名后有 <T>，声明了泛型类型参数
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // 使用非泛型函数
    reg_fn(S(A));          // 具体类型
    gen_spec_t(SGen(A));   // 隐式指定类型参数 A
    gen_spec_i32(SGen(6)); // 隐式指定类型参数 i32

    // 为 generic() 显式指定类型参数 char
    generic::<char>(SGen('a'));

    // 为 generic() 隐式指定类型参数 char
    generic(SGen('c'));
}