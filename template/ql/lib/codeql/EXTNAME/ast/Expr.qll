private import AstNodes
private import internal.AstNodes
private import internal.TreeSitter
private import internal.Expr
{% for expression in ts.Expr %}
private import internal.{{ expression }}
{%- endfor %}

/**
 * An expression in the AST.
 */
final class Expr extends AstNode instanceof ExprImpl { }
{% for expression in ts.Expr %}
/**
 * A {{ expression }} expression in the AST.
 */
final class {{ expression }} extends Expr instanceof {{ expression }}Impl { }
{%- endfor %}