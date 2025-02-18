{%- if ts.misc.size == 0  -%}
/**
 * No misc statements are not supported in `{{ config.name }}` code.
 */
{%- else %}
/**
 * Misc statements for AST nodes.
 */
private import codeql.{{ config.name }}.ast.AstNodes
private import AstNodes
private import TreeSitter

{% for misc in ts.misc %}
/**
 *  A {{ misc }} unknown statement.
 */
class {{ misc }}Impl extends AstNode, T{{ misc }} {
  private {{ config.name | upcase }}::{{ misc }} misc;

  override string getAPrimaryQlClass() { result = "?? {{ misc }} ??" }

  {{ misc }}Impl() { this = T{{ misc }}(misc) }
}
{% endfor %}
{%- endif %}
