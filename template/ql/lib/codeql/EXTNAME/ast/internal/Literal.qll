private import codeql.{{ config.name }}.ast.AstNodes
private import AstNodes
private import TreeSitter

/**
 * Literal statements.
 */
class LiteralsImpl extends AstNode, TLiterals {
  override string getAPrimaryQlClass() { result = "Literals" }

  /** Get the value of the literal */
  abstract string getValue();
}


{% for litab in ts.literalabstracts %}
/**
 *  A {{ litab }} statement.
 */
class {{ litab }}Impl extends LiteralsImpl, T{{ litab }} {
  private {{ config.name | upcase }}::{{ litab }} literal;

  override string getAPrimaryQlClass() { result = "{{ litab }}" }

  {{ litab }}Impl() { this = T{{ litab }}(literal) }

  override string toString() { result = literal.toString() }

  /** Get the value of the literal */
  override string getValue() { result = literal.getValue() }
}
{% endfor %}

{% for literal in ts.literals %}
/**
 *  A {{ literal }} statement.
 */
class {{ literal }}Impl extends LiteralsImpl, T{{ literal }} {
  private {{ config.name | upcase }}::{{ literal }} literal;

  override string getAPrimaryQlClass() { result = "{{ literal }}" }

  {{ literal }}Impl() { this = T{{ literal }}(literal) }

  override string toString() { result = literal.toString() }

  /** Get the value of the literal */
  override string getValue() { result = literal.getValue() }
}
{% endfor %}
