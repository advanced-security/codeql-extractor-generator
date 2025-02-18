private import codeql.{{ config.name }}.ast.AstNodes
private import AstNodes
private import TreeSitter

/**
 * A Bicep expression.
 */
class ExprImpl extends AstNode, TExpr {
  override string getAPrimaryQlClass() { result = "Expr" }
}
{% for expression in ts.expressions %}
/**
 *  A {{ expression }} expression.
 */
class {{ expression }}Impl extends ExprImpl, T{{ expression }} {
  private {{ config.name | upcase }}::{{ expression }} expression;

  override string getAPrimaryQlClass() { result = "{{ expression }}" }

  {{ expression }}Impl() { this = T{{ expression }}(expression) }

  override string toString() { result = expression.toString() }
}
{% endfor %}
