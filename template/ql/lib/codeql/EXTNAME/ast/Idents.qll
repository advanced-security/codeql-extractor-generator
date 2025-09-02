{%- if ts.Idents.size == 0  %}
/**
 *  No Idents AST Nodes
 */
{%- else %}
/**
 * Idents in the AST.
 */
private import AstNodes
private import Expr
// Internal imports
private import internal.AstNodes
private import internal.TreeSitter
private import internal.Idents
{%- for ident in ts.Idents %}
private import internal.{{ ident }}
{%- endfor %}

/**
 * An expression in the AST.
 */
final class Idents extends IdentsImpl {
    override string getName() { result = "" }
}

{%- for ident in ts.Idents %}
/**
 * A {{ ident }} expression in the AST.
 */
final class {{ ident }} extends Idents instanceof {{ ident }}Impl { }
{%- endfor %}
{%- endif %}