/**
 * Literals in the AST.
 */

private import AstNodes
private import internal.AstNodes
private import internal.TreeSitter
private import internal.Literals

/**
 * A literal in the AST.
 */
final class Literals extends AstNode instanceof LiteralsImpl { }

{% for literal in ts.Literals %}
/**
 * A {{ literal }} literal in the AST.
 */
final class {{ literal }} extends Literals instanceof {{literal }}Impl {
    {%- if literal == "Number" or literal == "Int" or literal == "Integer" %}
    /** Get the value as a int */
    int getNumber() { result = super.getValue().toInt() }
    {%- elsif literal == "Float" %}
    float getFloat() { result = super.getValue().toFloat() }
    {%- elsif literal == "String" %}
    string getString() { result = super.getValue() }
    {%- endif %}
}
{%- endfor %}