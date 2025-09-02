{%- if ts.Misc.size == 0  %}
/**
 *  No misc AST Nodes
 */
{%- else %}
private import AstNodes
{%- for misc in ts.Misc %}
private import internal.{{ misc }}
{%- endfor %}


{%- for misc in ts.Misc %}
/**
 *  A {{ misc }} unknown AST node.
 */
class {{ misc }} extends AstNode instanceof {{ misc }}Impl { }
{%- endfor %}
{%- endif %}
