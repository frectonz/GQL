use gitql_ast::environment::Environment;
use gitql_ast::expression::Expression;
use gitql_ast::object::GitQLObject;
use gitql_ast::object::Group;
use gitql_ast::object::Row;

use crate::engine_evaluator::evaluate_expression;

pub trait DataProvider {
    fn provide(
        &self,
        env: &mut Environment,
        table: &str,
        fields_names: &[String],
        titles: &[String],
        fields_values: &[Box<dyn Expression>],
    ) -> GitQLObject;
}

pub fn select_values(
    env: &mut Environment,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let mut group = Group { rows: vec![] };
    let mut values = Vec::with_capacity(fields_values.len());

    for value in fields_values.iter() {
        let evaluated = evaluate_expression(env, value, titles, &values)?;
        values.push(evaluated);
    }

    group.rows.push(Row { values });
    Ok(group)
}
