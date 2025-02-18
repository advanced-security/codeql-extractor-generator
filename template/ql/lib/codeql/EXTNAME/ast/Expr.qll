private import AstNodes
private import internal.AstNodes
private import internal.TreeSitter
private import internal.Expr

/**
 * An expression in the AST.
 */
final class Expr extends AstNode instanceof ExprImpl { }
{% for expression in ts.expressions %}
/**
 * A {{ expression }} expression in the AST.
 */
final class {{ expression }}Expr extends Expr instanceof {{ expression }}Impl { }
{%- endfor %}