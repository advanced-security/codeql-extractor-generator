{%- if ts.Loops.size == 0  %}
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
private import Idents

/**
 * Loop statements.
 */
abstract class LoopsImpl extends AstNode, TLoops {
  override string getAPrimaryQlClass() { result = "Loops" }

  abstract ExprImpl getCondition();

  abstract ExprImpl getBody();

  abstract IdentsImpl getInitializer();
}
{%- endif %}