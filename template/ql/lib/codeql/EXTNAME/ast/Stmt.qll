/**
 * Statement nodes in the AST.
 */
private import AstNodes
private import internal.AstNodes
private import internal.TreeSitter
private import internal.Stmt
// CFG
private import codeql.{{ config.name }}.CFG
private import codeql.{{ config.name }}.controlflow.internal.ControlFlowGraphImpl as CfgImpl

{%- if ts.block != 0 %}
{% for block in ts.block %}
/**
 * A {{ ts.block }}, a sequence of statements
 */
class {{ ts.block }} extends AstNode instanceof {{ ts.block }}Impl {
  /**
   * Get all the statements in a block
   */
  Stmt getStatements() {
    result = super.getStatements()
  }

  /**
   * Get a statement by index in a block
   */
  Stmt getStatement(int i) {
    result = super.getStatement(i)
  }
}
{%- endfor %}
{%- endif %}

{% if ts.statements.size != 0  %}
/**
 * A Stmt block
 */
class Stmt extends AstNode instanceof StmtImpl {
  /** Gets a control-flow node for this statement, if any. */
  CfgImpl::AstCfgNode getAControlFlowNode() { result.getAstNode() = this }

  /** Gets a control-flow entry node for this statement, if any */
  AstNode getAControlFlowEntryNode() { result = CfgImpl::getAControlFlowEntryNode(this) }
}
{%- endif %}
