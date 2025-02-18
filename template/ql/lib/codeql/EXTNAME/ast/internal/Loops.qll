{%- if ts.loops.size == 0  %}
/**
 * Loop statements are not supported in `{{ config.name }}` code.
 */
{%- else %}
/**
 * Loop statements for AST nodes.
 */
private import codeql.{{ config.name }}.ast.AstNodes
private import AstNodes
private import TreeSitter
private import Expr

/**
 * Loop statements.
 */
abstract class LoopsImpl extends AstNode, TLoops {
  override string getAPrimaryQlClass() { result = "Loops" }

  abstract ExprImpl getCondition();

  abstract ExprImpl getBody();

  // abstract Identifier getInitializer();
}
{% for loop in ts.loops %}
/**
 *  A {{ loop }} statement.
 */
class {{ loop }}Impl extends LoopsImpl, T{{ loop }} {
  private {{ config.name | upcase }}::{{ loop }} loop;

  override string getAPrimaryQlClass() { result = "{{ loop }}" }

  {{ loop }}Impl() { this = T{{ loop }}(loop) }

  override ExprImpl getCondition() { toTreeSitter(result) = loop.getCondition() }

  override ExprImpl getBody() { toTreeSitter(result) = loop.getBody() }

}
{% endfor %}
{%- endif %}