{%- if ts.conditionals.size == 0  %}
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
class ConditionalExprImpl extends ExprImpl, TConditionalExpr {
  override string getAPrimaryQlClass() { result = "Conditional" }

  abstract ExprImpl getCondition();
}

{% for condition in ts.conditionals %}
/**
 *  A {{ condition }} statement.
 */
class {{ condition }}Impl extends ConditionalExprImpl, T{{ condition }} {
  private {{ config.name | upcase }}::{{ condition }} condition;

  override string getAPrimaryQlClass() { result = "{{ condition }}" }

  {{ condition }}Impl() { this = T{{ condition }}(condition) }

  override ExprImpl getCondition() { toTreeSitter(result) = condition.getCondition() }
}
{% endfor %}
{%- endif %}