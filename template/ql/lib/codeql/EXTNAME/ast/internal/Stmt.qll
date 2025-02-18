private import codeql.{{ config.name }}.ast.AstNodes
private import AstNodes
private import TreeSitter
private import Expr

{%- if ts.block != 0 %}
{% for block in ts.block %}
/**
 * A {{ ts.block }}, a sequence of statements
 */
class {{ ts.block }}Impl extends ExprImpl, T{{ ts.block }} {
  private {{ config.name | upcase }}::{{ ts.block }} block;
  override string getAPrimaryQlClass() { result = "{{ ts.block }}" }

  {{ ts.block }}Impl() { this = T{{ ts.block }}(block) }

  /**
   * TODO: Update implementation if needed
   */
  StmtImpl getStatements() {
    toTreeSitter(result) = block.getChild(_)
  }

  /**
   * TODO: Update implementation if needed
   */
  StmtImpl getStatement(int i) {
    toTreeSitter(result) = block.getChild(i)
  }
}
{%- endfor %}
{%- endif %}

class StmtImpl extends AstNode, TStmt {
  override string getAPrimaryQlClass() { result = "Stmt" }
}
