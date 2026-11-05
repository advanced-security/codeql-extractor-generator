{%- if ts.Loops.size == 0  %}
/**
 * Loop statements are not supported in `{{ config.name }}` code.
 */
{%- else %}
/**
 * Loop statements for AST nodes.
 */
private import AstNodes
private import Stmts
private import Expr
private import Idents
// Internal imports
private import internal.Loops
{%- for loop in ts.Loops %}
private import internal.{{ loop }}
{%- endfor %}

/**
 * Loop statements implementation.
 */
class Loops extends Stmts instanceof LoopsImpl {
  /**
   *  Get the condition of the loop.
   */
  abstract Expr getCondition();

  /**
   *  Get the body of the loop.
   */
  abstract Stmts getBody();

  /**
   *  Get the initializer of the loop, if any.
   */
  abstract Idents getInitializer();
}
{% for loop in ts.Loops %}
/**
 *  A {{ loop }} statement.
 */
final class {{ loop }} extends Loops instanceof {{ loop }}Impl { 
  override Expr getCondition() { result = {{ loop }}Impl.super.getCondition() }

  override Stmts getBody() { result = {{ loop }}Impl.super.getBody() }

  override Idents getInitializer() { result = {{ loop }}Impl.super.getInitializer() }
}
{% endfor %}
{%- endif %}
