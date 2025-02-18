{%- if ts.loops.size == 0  %}
/**
 * Loop statements are not supported in `{{ config.name }}` code.
 */
{%- else %}
/**
 * Loop statements for AST nodes.
 */
private import AstNodes
private import internal.Loops
private import Stmt
private import Expr

/**
 * Loop statements implementation.
 */
final class Loops extends Stmt instanceof LoopsImpl { }
{% for loop in ts.loops %}
/**
 *  A {{ loop }} statement.
 */
final class {{ loop }} extends Loops instanceof {{ loop }}Impl { 
  Expr getCondition() { result = super.getCondition() }

  Expr getBody() { result = super.getBody() }

}
{% endfor %}
{%- endif %}
