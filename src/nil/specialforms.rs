use std::collections::HashMap;

type Callback = fn(Vec<Expression>, &mut Scope) -> Option<()>;
//user def fn call speial function that calls eval

pub struct SpecialForms {
    map: HashMap<String, Callback>
}

fn run(specialforms: &SpecialForms, scope: &Scope) {

}
