/**
 * Statement nodes in the AST.
 */
private import AstNodes
private import internal.AstNodes
private import internal.TreeSitter
private import internal.Stmts
private import internal.StmtSequence
// Sequence Statements
{%- for stmt in ts.StmtSequence %}
private import internal.{{ stmt }}
{%- endfor %}
// Statements
{%- for stmt in ts.Stmts %}
private import internal.{{ stmt }}
{%- endfor %}
// CFG
private import codeql.{{ config.name }}.CFG
private import codeql.{{ config.name }}.controlflow.internal.ControlFlowGraphImpl as CfgImpl

/**
 * A Stmt block
 */
final class Stmts extends AstNode instanceof StmtsImpl {
  /** Gets a control-flow node for this statement, if any. */
  CfgImpl::AstCfgNode getAControlFlowNode() { result.getAstNode() = this }
  
  /** Gets a control-flow entry node for this statement, if any */
  AstNode getAControlFlowEntryNode() { result = CfgImpl::getAControlFlowEntryNode(this) }
}

/**
 *  Represents a sequents of statements.
 */
final class StmtSequence extends AstNode instanceof StmtSequenceImpl {
  Stmts getStmts() {
    result = StmtSequenceImpl.super.getStmts()
  }

  Stmts getStmt(int index) {
    result = StmtSequenceImpl.super.getStmt(index)
  }
}

{% for stmt in ts.StmtSequence %}
/**
 * A {{ stmt }} statement sequence
 */
class {{ stmt }} extends StmtSequence instanceof {{ stmt }}Impl { }
{%- endfor %}

{% for stmt in ts.Stmts %}
/**
 * A {{ stmt }} statement
 */
class {{ stmt }} extends Stmts instanceof {{ stmt }}Impl { }
{%- endfor %}
