private import AstNodes
private import internal.Misc

{%- if ts.misc.size != 0  %}
{% for misc in ts.misc %}
/**
 *  A {{ misc }} unknown AST node.
 */
class {{ misc }} extends AstNode instanceof {{ misc }}Impl { }
{% endfor %}
{%- endif %}
