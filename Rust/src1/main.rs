mod hello_world;
mod primitives;
mod structures;
mod variable_bindings;
mod types;
mod conversion;
mod expressions;
mod flow_of_control;
mod functions;
mod modules;

fn main() {
    hello_world::say_hello(String::from("Donghua Chen"));
    hello_world::formatted_print(2,3);
    hello_world::use_fmt();
    hello_world::show_color();

    primitives::use_primitives();
    primitives::use_tuple();
    primitives::use_array();

    structures::use_structure();
    structures::use_enums();
    structures::use_constants();

    variable_bindings::test_mutability();
    variable_bindings::test_scope_and_shadowing();
    variable_bindings::test_declare_first();
    variable_bindings::test_freezing();

    types::test_casting();
    types::test_literals();
    types::test_inference();
    types::test_aliasing();

    conversion::test_from_to();
    conversion::test_tryfrom_tryinto();
    conversion::test_to_from();

    expressions::test_expressions();

    flow_of_control::test_if();
    flow_of_control::test_loop();
    flow_of_control::test_while();
    flow_of_control::test_for();
    flow_of_control::test_match();
    flow_of_control::test_if_let();
    flow_of_control::test_while_let();

    functions::test_method();
    functions::test_closures();
    functions::test_higher_order_func();
    functions::test_diverging();

    modules::test_modules();

}
