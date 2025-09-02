private import {{ config.name }}
private import codeql.{{ config.name }}.AST
private import Completion
private import ControlFlowGraphImpl

/**
 * A scope in the control flow graph.
 */
abstract private class CfgScopeImpl extends AstNode {
  /** Holds if `first` is executed first when entering scope. */
  abstract predicate scopeFirst(AstNode first);

  /** Holds if scope is exited when `last` finishes with completion `c`. */
  abstract predicate scopeLast(AstNode last, Completion c);
}

final class CfgScope = CfgScopeImpl;

{%- for stmt in ts.StmtSequence %}
/**
 * A {{ stmt }} statement sequence
 */
final class {{ stmt }}Scope extends CfgScopeImpl, {{ stmt }} {
  override predicate scopeFirst(AstNode first) { first(this.getStmt(0), first) }

  override predicate scopeLast(AstNode last, Completion c) { last(this, last, c) }
}
{%- endfor %}

/**
 * A Call Definition Scope
 */
final class CallDefinitionScope extends CfgScopeImpl instanceof CallDefinition {
  override predicate scopeFirst(AstNode first) { first(CallDefinition.super.getStmt(0), first) }

  override predicate scopeLast(AstNode last, Completion c) { last(this, last, c) }

  AstNode getStmt(int index) {
    result = CallDefinition.super.getStmt(index)
  }
}