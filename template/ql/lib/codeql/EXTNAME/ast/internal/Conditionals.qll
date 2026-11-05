{%- if ts.Conditionals.size == 0  %}
/**
 * Conditional statements are not supported in `{{ config.name }}` code.
 */
{%- else %}
/**
 * Conditional statements.
 */
private import codeql.{{ config.name }}.ast.AstNodes
private import AstNodes
private import TreeSitter
private import Expr

/**
 * Conditional statements.
 */
class ConditionalsImpl extends ExprImpl, TConditionals {
  override string getAPrimaryQlClass() { result = "Conditional" }

  abstract ExprImpl getCondition();
}
{%- endif %}